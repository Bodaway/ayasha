use crate::lacrosse_v3_protocol;
use crate::sensor::{models::*, repository::*};
use crate::serial_com::models::*;
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("database access error for {}", description))]
    DatabaseAccessError {
        description: String,
        source: diesel::result::Error,
    },

    #[snafu(display("decrypt error for protocol {}", protocol))]
    DecryptError {
        protocol: String,
        source: lacrosse_v3_protocol::LacrosseError,
    },

    #[snafu(display("Not implement error"))]
    NotImplementedError,
}
type Result<T, E = Error> = std::result::Result<T, E>;

type Provider = &'static dyn Fn() -> SensorProvider;

pub fn frame_received(frame: Frame, provider_of_sensor: Provider) {
    match frame {
        Frame::DebugFrame(df) => debug_case(df, provider_of_sensor),
        Frame::RfLinkFrame(_rf) => Err(Error::NotImplementedError),
    };
}

fn debug_case(df: DebugFrame, prov: Provider) -> Result<()> {
    let data = lacrosse_v3_protocol::decrypt(df.pulses.as_ref()).context(DecryptError{protocol: "lacrosse_v3_protocol"})?;
    println!(
        "id:{}, temperature:{}, humidity:{}",
        data.sensor_id,
        data.temperature.to_string(),
        data.humidity.to_string()
    );
    let id_temp = (data.sensor_id + 10000) as SensorId;
    let id_hum = (data.sensor_id + 100000) as SensorId;
    let state_temp =
        InsertableSensorState::new((data.sensor_id + 10000) as SensorId, data.temperature);
    let state_hum =
        InsertableSensorState::new((data.sensor_id + 100000) as SensorId, data.humidity as f32);
    let repo = prov();

    if !(sensor_exist(id_temp, prov)?) {
       Sensor::new(id_temp,SensorType::Temperature,"Celcius".into()); 
    }

    if !(sensor_exist(id_hum, prov)?) {
       Sensor::new(id_hum,SensorType::Humidity,"%".into()); 
    }

    (repo.insert_sensor_state)(&state_temp).context(DatabaseAccessError {
        description: "method insert_sensor_state as fail",
    })?;
    (repo.insert_sensor_state)(&state_hum).context(DatabaseAccessError {
        description: "method insert_sensor_state as fail",
    })?;


    Ok(())
}

fn sensor_exist(id: SensorId, prov: Provider) -> Result<bool> {
    let prov = prov();
    let sensors = (prov.get_all_sensor)().context(DatabaseAccessError {
        description: "method get_all_sensor as fail",
    })?;

    let exist = sensors.into_iter().find(|s| s.id == id).is_some();
    Ok(exist)
}
