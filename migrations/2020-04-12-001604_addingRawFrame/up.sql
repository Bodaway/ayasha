-- Your SQL goes here
create table RawFrame (
    frame_id INTEGER NOT NULL PRIMARY KEY,
    raw_data text not null,
    dt_occurs Timestamp not null );