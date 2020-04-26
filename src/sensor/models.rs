use std::fmt::Display;
use ::serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use crate::schema::*;

pub type SensorId = i32;

#[derive(Serialize, Deserialize, Debug)]
pub enum SensorType {
    Temperature,
    PressionAtmos
}

impl Display for SensorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SensorType::Temperature     => write!(f,"Temperature"),
            SensorType::PressionAtmos   => write!(f,"PressionAtmos"),
        }
    } 
}

#[derive(Identifiable,Queryable, Associations, PartialEq, Debug)]
#[table_name = "location"]
pub struct Location {
    pub id : i32,
    pub name : String
}

#[derive(Insertable,Debug)]
#[table_name = "location"]
pub struct InsertableLocation {
    pub name : String
}

impl InsertableLocation {
    pub fn new(name_of_location : &str) -> InsertableLocation {
        InsertableLocation{name: name_of_location.to_string()}
    }
}

#[derive(Identifiable,Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Location)] 
#[table_name = "sensor"]
pub struct Sensor {
    pub id: SensorId,
    pub name : String,
    pub location_id : Option<i32>,
    pub sensor_type : String,
    pub unit : String,
}

impl Sensor {
    pub fn create_state(&self, value : f32) -> SensorState {
        SensorState::new(self.id,value)
    }
}

#[derive(Insertable,Debug)]
#[table_name = "sensor"]
pub struct InsertableSensor {
    pub name : String,
    pub location_id : Option<i32>,
    pub sensor_type : String,
    pub unit : String,
}

impl InsertableSensor {
    pub fn new(sensor_name : &str, sensor_location : Option<i32>, type_of_sensor : SensorType, sensor_unit : &str) -> InsertableSensor {
        InsertableSensor {name : sensor_name.to_string(),location_id : sensor_location, sensor_type : type_of_sensor.to_string(),unit : sensor_unit.to_string()}
    }
}

#[derive(Insertable,Queryable,Serialize, Deserialize, Debug)]
#[table_name = "sensor_state"]
pub struct SensorState {
    pub sensor_id : SensorId,
    pub dt_update: NaiveDateTime,
    pub sensor_value: f32,
}

impl SensorState {
    pub fn new(id_of_sensor : SensorId, value : f32) -> SensorState {
        SensorState{sensor_id : id_of_sensor, sensor_value : value, dt_update : chrono::Local::now().naive_local() }
    }
}
