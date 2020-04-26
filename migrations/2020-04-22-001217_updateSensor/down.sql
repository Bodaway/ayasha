-- This file should undo anything in `up.sql`
drop table sensor;
drop table location;
CREATE TABLE sensor(
    sensor_id INTEGER PRIMARY KEY not null,
    name text not null,
    location text not null,
    sensor_type text not null,
    unit text not null);
