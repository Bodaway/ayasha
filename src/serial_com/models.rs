use crate::schema::*;

#[derive(Insertable)]
#[table_name = "raw_frame_info"]
pub struct InsertableRawFrameInfo {
    raw_data : String,
    dt_occurs: chrono::NaiveDateTime
}

pub struct RawFrame{
    data : String,
    timestamp : chrono::NaiveDateTime   
}

pub struct RfLinkFrame{
    data : String ,
    timestamp : chrono::NaiveDateTime   
}

pub struct DebugFrame {
    pulses_number : i32,
    pub pulses : String,
    timestamp : chrono::NaiveDateTime   
}

pub enum Frame{
    RfLinkFrame(RfLinkFrame),
    DebugFrame(DebugFrame)
}

impl DebugFrame {
    pub fn from_raw(raw : &RawFrame) -> DebugFrame {
        let raw_vec = raw.to_vec();
        let pulses_number = (&raw_vec[3][7..]).parse::<i32>().expect("fuck") ;
        let pulses_str = (&raw_vec[4][13..]).to_string();

        DebugFrame{pulses_number : pulses_number, pulses: pulses_str,timestamp : raw.timestamp}
    }
}

use std::error::Error;
impl RawFrame{
    pub fn new(data : &str) -> RawFrame {
        RawFrame{data : String::from(data), timestamp : chrono::Local::now().naive_local()}
    }
    pub fn from_string(data : String) -> RawFrame {
        RawFrame::new(data.as_str())
    }

    pub fn to_vec(&self) -> Vec<&str> {
        self.data.split(';').collect::<Vec<&str>>()
    }

    pub fn is_debug(&self) -> bool {
        let vec = self.to_vec();       
        vec.len() > 2 && vec[2] == "DEBUG"
    }

    pub fn from_utf8(data : Vec<u8>) -> Result<RawFrame,Box<dyn Error>> {
        Ok(RawFrame::from_string(String::from_utf8(data)?))
    }
    
    pub fn to_raw_frame_info(&self) -> InsertableRawFrameInfo {
        InsertableRawFrameInfo{raw_data: self.data.clone(),dt_occurs:self.timestamp}
    }
}

impl RfLinkFrame{
    pub fn from_raw(raw : &RawFrame) -> RfLinkFrame {
        RfLinkFrame{data: raw.data.clone(), timestamp: raw.timestamp}
    }
}
