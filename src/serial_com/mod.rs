extern crate serial;

use std::io;
use std::time::Duration;
use std::thread;

use serial::prelude::*;

pub mod models;
pub mod repository;

use crate::serial_com::models::*;
use crate::serial_com::repository::*;
use crate::connection::*;

pub fn start_listen(on_frame_receive : Box<dyn Fn(Frame)>) {

    loop {
        let result = listen(&on_frame_receive);
        match result {
            Ok(_x) => (),
            Err(e) => debug!("serial com error {}", e )
        }
    }
}

pub fn listen(on_data_receive: &dyn Fn(Frame)) -> io::Result<()> {
    let port = &mut serial::open("/dev/ttyACM0")?; //SERIAL_PORT 
    port.reconfigure(&|settings| { 
    settings.set_baud_rate(serial::Baud57600)?;
    settings.set_char_size(serial::Bits8);
    settings.set_parity(serial::ParityNone);
    settings.set_stop_bits(serial::Stop1);
    settings.set_flow_control(serial::FlowNone);
    Ok(())
    })?;

    port.set_timeout(Duration::from_millis(100000))?;

    read_line(port)?;

    let debug_engaged = set_debug_mode(port)?;
    debug!("debug engaged is {}", debug_engaged);
    

    loop{
        let frame = read_line(port)?;

        let get_repo = &||FrameProvider::new(&establish); 
        let irf = frame.to_raw_frame_info();
        (get_repo().insert_frame)(&irf).expect("raw frame insertion fail");

        let frame_result = match frame.is_debug() {
                                true =>{
                                    let debug_frame = DebugFrame::from_raw(&frame);
                                    Frame::DebugFrame(debug_frame)
                                },
                                false => {
                                    let rf_frame = RfLinkFrame::from_raw(&frame);
                                    Frame::RfLinkFrame(rf_frame)
                                }
                            };
        on_data_receive(frame_result); //call extern function
    }
}


fn read_line<T: SerialPort>(port : &mut T) -> io::Result<RawFrame> {
    // 10 = line feed
    let mut input: Vec<u8> = Vec::with_capacity(500);
    let mut buf = [0 as u8];
    while buf[0] != 10 {
        port.read(&mut buf)?;
        input.push(buf[0]);
    }
    let result = RawFrame::from_utf8(input).expect("Found invalid UTF-8");
    Ok(result)
}

fn set_debug_mode<T: SerialPort>(port : &mut T) -> io::Result<bool> {
    port.write("10;rfdebug=on;\r\n".as_bytes())?;
    let response = read_line(port)?;
    match response {
        res if res.is_debug() => Ok(true), //"20;01;RFDEBUG=ON;"
        _ => {
            Ok(false)
        }

    }
}
