#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate snafu;

extern crate dotenv;

#[macro_use]
extern crate log;
extern crate syslog;
#[macro_use]
extern crate lazy_static;

use log::LevelFilter; //SetLoggerError
use rocket::http::RawStr;
use syslog::{BasicLogger, Facility, Formatter3164};

use std::thread;

mod connection;
mod event_hub;
mod lacrosse_v3_protocol;
mod schema;
mod sensor;
mod serial_com;
mod traitement_recurent;
mod frame_process;

use serial_com::models::*;
use crate::connection::*;

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
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .expect("unable to box logger");
{
        let locker = DB_POOL.clone();
        let pool = locker.lock().unwrap();
        let connection = pool.get().unwrap();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("diesel migration fail");
}
    use traitement_recurent::*;

    thread::spawn(move || {
        traitement_recurent();
    });

    use serial_com::*;
    thread::spawn(move || {
        let on_frame_receive = |f: Frame| {
            let result = frame_process::frame_received(f,&|| sensor::repository::SensorProvider::new());
            match result {
                Err(ex) => error!("{}",ex.to_string()),
                _ => ()
            };
        };
        start_listen(Box::new(on_frame_receive));
    });

    info!("hello world");

    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                get_sensors_values,
                covid19_exposed_today,
                get_all_event,
                get_no_read_event,
                set_event_read,
                create_location,
                get_last_sensor_state,
                get_location_status,
                assign_sensor
            ],
        )
        .launch();

}

// ROCKET

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[put("/setEventToRead/<id>")]
fn set_event_read(id: &RawStr) {
    let id_int = id.as_str().parse::<i32>().expect("Parse Id error");

    use crate::event_hub::models::*;
    use crate::event_hub::repository::*;
    use crate::event_hub::*;
    let get_repo = &|| EventProvider::new();

    (get_repo().set_event_to_read)(&id_int).expect("set_event_read update fail");
}

#[get("/covid19ExposedToday")]
fn covid19_exposed_today() {
    use crate::event_hub::models::*;
    use crate::event_hub::repository::*;
    use crate::event_hub::*;
    let get_repo = &|| EventProvider::new();

    let iev = InsertableEvent::new(
        EventType::Covid19ExposedToday,
        "UserInput".to_string(),
        None,
        None,
        None,
    );

    let iev_read = InsertableEvent {
        read_flag: true,
        ..iev
    };

    (get_repo().insert_event)(&iev_read).expect("covid19_exposed_today exposed insertion fail");
}

#[get("/getSensorValues")]
fn get_sensors_values() -> String {
    use sensor::*;

    let provider_sensor = repository::SensorProvider::new();
    let result_table = (provider_sensor.get_all_sensor_state)().expect("Fuck dont work");

    serde_json::to_string(&result_table).expect("serialisation fail")

    // currentState.to_string()
}

#[get("/getAllEvents")]
fn get_all_event() -> String {
    use event_hub::*;

    let provider_event = repository::EventProvider::new();
    let result_table = (provider_event.get_all_event)().expect("Fuck dont work");

    serde_json::to_string(&result_table).expect("serialisation fail")
}

#[get("/getNoReadEvents")]
fn get_no_read_event() -> String {
    use event_hub::*;

    let provider_event = repository::EventProvider::new();
    let result_table = (provider_event.get_event_no_read)().expect("Fuck dont work");

    serde_json::to_string(&result_table).expect("serialisation fail")
}

#[get("/getLastSensorState")]
fn get_last_sensor_state() -> String {
    use sensor::*;

    let provider_sensor = repository::SensorProvider::new();
    let result_table = (provider_sensor.get_last_sensor_state)();
    match result_table {
        Err(e) => e.to_string(),
        Ok(data) => serde_json::to_string(&data).expect("serialisation fail"),
    }
}

use crate::sensor::models::*;

#[put("/createLocation/<name>")]
fn create_location(name: &RawStr) {
    use sensor::*;

    let get_repo = &|| repository::SensorProvider::new();
    let iloca = InsertableLocation::new(name.as_str());
    (get_repo().insert_location)(iloca).expect("location insertion fail");
}

#[put("/assignSensor/<sensor_id>/<location_id>")]
fn assign_sensor(sensor_id : SensorId, location_id : LocationId){
    use sensor::*;
 let get_repo = &||repository::SensorProvider::new();
    (get_repo().update_sensor_location)(sensor_id,location_id).expect("location insertion fail");

}

#[get("/getLocationStatus")]
fn get_location_status() -> String {
    use sensor::hl_models::*;
    use sensor::*;

    let provider_sensor = repository::SensorProvider::new();
    let result_table = (provider_sensor.get_all_location_status)();
    match result_table {
        Err(e) => e.to_string(),
        Ok(data) => {
            let locations = LocationStatus::get_all_status(data);
            serde_json::to_string(&locations).expect("serialisation fail")
        }
    }
} 

