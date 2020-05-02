use crate::schema::*;
use ::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use std::fmt::Display;

pub type SensorId = i32;
pub type LocationId = i32;
pub type StateId = i32;

#[derive(Serialize, Deserialize, Debug)]
pub enum SensorType {
    Temperature,
    PressionAtmos,
}

impl Display for SensorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SensorType::Temperature => write!(f, "Temperature"),
            SensorType::PressionAtmos => write!(f, "PressionAtmos"),
        }
    }
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, PartialEq, Debug)]
#[table_name = "location"]
pub struct Location {
    pub id: LocationId,
    pub name: String,
}

#[derive(Insertable, Debug)]
#[table_name = "location"]
pub struct InsertableLocation {
    pub name: String,
}

impl InsertableLocation {
    pub fn new(name_of_location: &str) -> InsertableLocation {
        InsertableLocation {
            name: name_of_location.to_string(),
        }
    }
}

#[derive(
    Identifiable, Insertable, Queryable, Associations, Serialize, Deserialize, PartialEq, Debug,
)]
#[belongs_to(Location)]
#[table_name = "sensor"]
pub struct Sensor {
    pub id: SensorId,
    pub location_id: Option<LocationId>,
    pub sensor_type: String,
    pub unit: String,
    pub is_active: bool,
}

impl Sensor {
    pub fn create_state(&self, value: f32) -> InsertableSensorState {
        InsertableSensorState::new(self.id, value)
    }
}
#[derive(
    Identifiable, Queryable, QueryableByName, Associations, Serialize, Deserialize, PartialEq, Debug,
)]
#[belongs_to(Sensor)]
#[table_name = "sensor_state"]
pub struct SensorState {
    pub id: StateId,
    pub sensor_id: SensorId,
    pub dt_update: NaiveDateTime,
    pub sensor_value: f32,
}

#[derive(Insertable, Queryable, QueryableByName, Associations, Debug)]
#[belongs_to(Sensor)]
#[table_name = "sensor_state"]
pub struct InsertableSensorState {
    pub sensor_id: SensorId,
    pub dt_update: NaiveDateTime,
    pub sensor_value: f32,
}

impl InsertableSensorState {
    pub fn new(id_of_sensor: SensorId, value: f32) -> InsertableSensorState {
        InsertableSensorState {
            sensor_id: id_of_sensor,
            sensor_value: value,
            dt_update: chrono::Local::now().naive_local(),
        }
    }
}
