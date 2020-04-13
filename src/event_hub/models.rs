use crate::schema::*;
use crate::sensor::models::SensorId;
use ::serde::{Deserialize, Serialize};
use chrono::*;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub enum EventType {
    LowBattery,
    LostSignal,
    OpenWindows,
    Covid19ExposedToday,
    Covid19withoutExposure,
}

impl EventType {
    pub fn from_string(input : &str) -> EventType {
        match input {
            "LowBattery" => EventType::LowBattery ,
            "LostSignal" => EventType::LostSignal ,
            "OpenWindows" => EventType::OpenWindows ,
            "Covid19ExposedToday" => EventType::Covid19ExposedToday ,
            "Covid19withoutExposure" => EventType::Covid19withoutExposure ,
            _ => panic!("convert impossible")
        }
    }
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::LowBattery => write!(f, "LowBattery"),
            EventType::LostSignal => write!(f, "LostSignal"),
            EventType::OpenWindows => write!(f, "OpenWindows"),
            EventType::Covid19ExposedToday => write!(f, "Covid19ExposedToday"),
            EventType::Covid19withoutExposure => write!(f, "Covid19withoutExposure"),
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Event {
    pub event_id: i32,
    pub event_type: String,
    pub sensor_source_id: Option<SensorId>,
    pub rules_name: String,
    pub event_value: Option<String>,
    pub event_value_definition: Option<String>,
    pub dt_occurs: chrono::NaiveDateTime,
    pub read_flag : bool
}

impl Event {
    pub fn get_notification_data(&self) -> (String,String) {
        match EventType::from_string(&self.event_type) {
            EventType::LowBattery => ("pile faible".to_string(),format!("capteur {} signal des pile faible.", self.sensor_source_id.as_ref().unwrap_or(&-1) )),
            EventType::LostSignal => ("".to_string(),"".to_string()),
            EventType::OpenWindows => ("Ouvrir les fênetres".to_string(),"penser à ouvrir les fenetres !".to_string()),
            EventType::Covid19ExposedToday => ("COVID-19".to_string(),"Il y a eu exposition aujourd'hui".to_string()),
            EventType::Covid19withoutExposure => ("COVID-19".to_string(),format!("pas d'exposition depuis {} jours", self.event_value.as_ref().unwrap_or(&String::from("-1")))),
        }
    }
}


#[derive(Insertable)]
#[table_name = "event"]
pub struct InsertableEvent {
    pub event_type: String,
    pub sensor_source_id: Option<SensorId>,
    pub rule_name: String,
    pub event_value: Option<String>,
    pub event_value_definition: Option<String>,
    pub dt_occurs: chrono::NaiveDateTime,
    pub read_flag : bool
}

impl InsertableEvent {
    pub fn new(
        type_of_event: EventType,
        rules_source_name: String,
        source_id: Option<SensorId>,
        value_of_event: Option<String>,
        value_definition: Option<String>    ) -> InsertableEvent {

        InsertableEvent {
            event_type: type_of_event.to_string(),
            sensor_source_id: source_id,
            rule_name: rules_source_name,
            dt_occurs: Local::now().naive_local(),
            event_value: value_of_event,
            event_value_definition: value_definition,
            read_flag: false
        }
    }
}
