use std::error::Error;

pub struct LaCrosseData {
    pub sensor_id : i32,
    pub temperature : f32,
    pub humidity : i32 
}

pub fn decrypt(raw_data : &str) -> Result<LaCrosseData,Box<dyn Error>> {
    //if pulse_number != "511" {warn!("pulse number different du standart LaCrosse 511 : {}", pulse_number)};

   let signal = raw_data.split(',').collect::<Vec<&str>>();

   let tuple_pulse = to_tuple_pulse(&signal)?;
   let binary_signal = binarize(tuple_pulse);

   let binary_frames = binary_signal.split("hhhh").into_iter().filter(|x| x.len() == 41).collect::<Vec<&str>>();
   if binary_frames.len() != 4 {warn!(" {} frames trouver au lieu des 4 prévu",binary_frames.len())}
   if binary_frames[0].len() != binary_frames[1].len() { }

    let w_frame = binary_frames[0];

    let id_bin = isize::from_str_radix( &w_frame[..8],2)? as i32;
    let temp_bin = &w_frame[12..24];
    let temp_val:f32 = (isize::from_str_radix(reverse_binary(temp_bin).as_str(), 2).unwrap() as f32) / 10.0 - 50.0 ;
    let hum_bin = &w_frame[25..32];
    let hum_val = isize::from_str_radix(reverse_binary(hum_bin).as_str(), 2)? as i32;



   Ok(LaCrosseData {sensor_id:id_bin,temperature:temp_val,humidity:hum_val})

}

fn to_tuple_pulse(signal : &Vec<&str>) -> Result<Vec<(i32,i32)>,Box<dyn Error>> {
    let mut i = 0;
    let mut done = false ;
    let mut tuple_pulse : Vec<(i32,i32)> = Vec::new();
    let ended_index = signal.len() -3;  


    while !done {
        let t1 = signal[i].parse::<i32>()?;
        let t2 = signal[i+1].parse::<i32>()?;
        tuple_pulse.push((t1,t2));
        
        if i == ended_index { done = true;}
        i = i+2;
    }
    Ok(tuple_pulse)

}

fn binarize(tuple_signal : Vec<(i32,i32)> ) -> String {
    tuple_signal.into_iter().map(|t| match t {
        (x,y) if x > 450 && y > 450 => "h",
        (x,y) if x > y => "0",
        _ => "1"
    }).collect::<Vec<&str>>().concat()
}

fn reverse_binary(frame : &str) -> String {
    let mut new_frame = String::new() ;

    for bit in frame.chars() {
        match bit {
            '1' => new_frame.push('0'),
            _ => new_frame.push('1')
        }
    }
    new_frame
}
