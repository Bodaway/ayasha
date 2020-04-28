-- This file should undo anything in `up.sql`
drop table sensor;
drop table sensor_state;

CREATE TABLE sensor(
    id INTEGER PRIMARY KEY not null,
    name text not null,
    location_id integer references location(loc_id),
    sensor_type text not null,
    unit text not null);

CREATE TABLE sensor_state(
    sensor_id INTEGER not null,
    dt_update Timestamp not null,
    sensor_value real not null,
	PRIMARY KEY (sensor_id,dt_update));
