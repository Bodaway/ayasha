-- Your SQL goes here
CREATE TABLE event (
    event_id INTEGER NOT NULL PRIMARY KEY,
    event_type text not null,
    sensor_source_id INTEGER,
    rule_name text not null,
    dt_occurs Timestamp not null );


-- insert into event
-- values (0,'LowBattery',0,'2020-02-10');

-- insert into event
-- values (1,'OpenWindows',null,'2020-03-05')

