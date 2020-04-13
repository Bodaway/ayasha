-- This file should undo anything in `up.sql`
drop table event;
CREATE TABLE event (
    event_id INTEGER NOT NULL PRIMARY KEY,
    event_type text not null,
    sensor_source_id INTEGER,
    rule_name text not null,
    dt_occurs Timestamp not null );