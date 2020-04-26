use crate::schema::*;
use crate::serial_com::models::*;
use diesel::*;

pub struct FrameProvider {
    pub insert_frame            : Box<dyn Fn(&InsertableRawFrameInfo) -> QueryResult<usize>>,
}

impl FrameProvider {
    pub fn new(connp :&'static dyn Fn() -> SqliteConnection ) ->  FrameProvider {
        FrameProvider{
            insert_frame : Box::new(move |irf: &InsertableRawFrameInfo| {
                diesel::insert_into(raw_frame_info::table).values(irf).execute(&connp())
                })
        }
    }
}
















