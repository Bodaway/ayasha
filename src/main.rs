#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
extern crate dotenv;

#[macro_use] extern crate log;
extern crate syslog;

use syslog::{Facility, Formatter3164, BasicLogger};
use log::{LevelFilter};//SetLoggerError
use rocket::http::RawStr;

use std::{thread};

mod connection;
mod event_hub;
mod schema;
mod sensor;
mod traitement_recurent;
mod serial_com;


embed_migrations!("./migrations");

fn main() {

    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "ayasha".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
            .map(|()| log::set_max_level(LevelFilter::Debug)).expect("unable to box logger");


    let connection = connection::establish();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).expect("diesel migration fail");

    use traitement_recurent::*;

    thread::spawn(move || {
        traitement_recurent();
    });

    use serial_com::*;
    thread::spawn(move || {
        start_listen();
    });

    
    info!("hello world");

    rocket::ignite()
        .mount("/", routes![index, get_sensors_values,covid19_exposed_today,get_all_event,get_no_read_event,set_event_read])
        .launch();
}


// ROCKET 

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[put("/setEventToRead/<id>")]
fn set_event_read(id : &RawStr) {
    let id_int = id.as_str().parse::<i32>().expect("Parse Id error");

    use crate::event_hub::models::*;
    use crate::event_hub::repository::*;
    use crate::event_hub::*;
    let get_repo = &||EventProvider::new(&connection::establish);

    (get_repo().set_event_to_read)(&id_int).expect("set_event_read update fail");
}


#[get("/covid19ExposedToday")]
fn covid19_exposed_today() {
    use crate::event_hub::models::*;
    use crate::event_hub::repository::*;
    use crate::event_hub::*;
    let get_repo = &||EventProvider::new(&connection::establish);

    let iev = InsertableEvent::new(EventType::Covid19ExposedToday, "UserInput".to_string(), None,None,None);

    let iev_read = InsertableEvent{read_flag : true, ..iev};

    (get_repo().insert_event)(&iev_read).expect("covid19_exposed_today exposed insertion fail");
}

#[get("/getSensorValues")]
fn get_sensors_values() -> String {
    use sensor::*;

    let provider_sensor = repository::SensorProvider::new(&connection::establish);
    let result_table = (provider_sensor.get_all_sensor_state)().expect("Fuck dont work");

    serde_json::to_string(&result_table).expect("serialisation fail")

    // currentState.to_string()
}

#[get("/getAllEvents")]
fn get_all_event() -> String {
    use event_hub::*;

    let provider_event = repository::EventProvider::new(&connection::establish);
    let result_table = (provider_event.get_all_event)().expect("Fuck dont work");

    serde_json::to_string(&result_table).expect("serialisation fail")
}


#[get("/getNoReadEvents")]
fn get_no_read_event() -> String {
    use event_hub::*;

    let provider_event = repository::EventProvider::new(&connection::establish);
    let result_table = (provider_event.get_event_no_read)().expect("Fuck dont work");
    
    serde_json::to_string(&result_table).expect("serialisation fail")
}
