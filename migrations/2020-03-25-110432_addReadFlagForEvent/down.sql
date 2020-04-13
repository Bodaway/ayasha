-- This file should undo anything in `up.sql`



alter table event rename to event_old;

CREATE TABLE event (
    event_id INTEGER NOT NULL PRIMARY KEY,
    event_type text not null,
    sensor_source_id INTEGER,
    rule_name text not null,
    event_value text,
    event_value_definition text,
    dt_occurs Timestamp not null );

insert into event(event_id,event_type,sensor_source_id,rule_name,event_value,event_value_definition,dt_occurs) 
select event_id,event_type,sensor_source_id,rule_name,event_value,event_value_definition,dt_occurs 
from event_old;

drop table event_old;