-- Your SQL goes here
CREATE TABLE sensor(
    sensor_id INTEGER PRIMARY KEY not null,
    name text not null,
    location text not null,
    sensor_type text not null,
    unit text not null);

CREATE TABLE sensor_state(
    sensor_id INTEGER not null,
    dt_update Timestamp not null,
    sensor_value real not null,
	PRIMARY KEY (sensor_id,dt_update));
