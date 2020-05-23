use crate::schema::*;
use crate::serial_com::models::*;
use diesel::*;
use crate::connection::*;

pub struct FrameProvider {
    pub insert_frame            : Box<dyn Fn(&InsertableRawFrameInfo) -> QueryResult<usize>>,
}

impl FrameProvider {
    pub fn new() ->  FrameProvider {
        FrameProvider{
            insert_frame : Box::new(move |irf: &InsertableRawFrameInfo| {
                let locker = DB_POOL.clone();
                let pool = locker.lock().unwrap();
                let conn = pool.get().unwrap();
                diesel::insert_into(raw_frame_info::table).values(irf).execute(&conn)
                })
        }
    }
}
















