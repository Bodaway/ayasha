-- Your SQL goes here
drop table sensor;

create table location(
	id integer primary key not null,
	name text not null);

CREATE TABLE sensor(
    id INTEGER PRIMARY KEY not null,
    name text not null,
    location_id integer references location(loc_id),
    sensor_type text not null,
    unit text not null);
