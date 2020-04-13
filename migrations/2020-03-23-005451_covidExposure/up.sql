-- Your SQL goes here
drop table event;
CREATE TABLE event (
    event_id INTEGER NOT NULL PRIMARY KEY,
    event_type text not null,
    sensor_source_id INTEGER,
    rule_name text not null,
    event_value text,
    event_value_definition text,
    dt_occurs Timestamp not null );