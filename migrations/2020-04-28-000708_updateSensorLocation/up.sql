drop table sensor;
drop table sensor_state;

CREATE TABLE sensor(
    id INTEGER PRIMARY KEY not null,
    location_id integer references location(loc_id),
    sensor_type text not null,
    unit text not null);
    
CREATE TABLE sensor_state(
    sensor_id INTEGER not null references sensor(id),
    dt_update Timestamp not null,
    sensor_value real not null,
	PRIMARY KEY (sensor_id,dt_update));
