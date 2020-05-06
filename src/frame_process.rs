use crate::lacrosse_v3_protocol;
use crate::sensor::{models::*, repository::*};
use crate::serial_com::models::*;
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("database access error : {}", source.to_string()))]
    DatabaseAccessError {
        source: diesel::result::Error,
    },

    #[snafu(display("error for protocol lacrosse : {}", source.to_string()))]
    LacrosseError {
        source: lacrosse_v3_protocol::LacrosseError,
    },

    #[snafu(display("Not implement error"))]
    NotImplementedError,
}
type Result<T, E = Error> = std::result::Result<T, E>;

type Provider = &'static dyn Fn() -> SensorProvider;

pub fn frame_received(frame: Frame, provider_of_sensor: Provider) -> Result<()> {
    match frame {
        Frame::DebugFrame(df) => debug_case(df, provider_of_sensor),
        Frame::RfLinkFrame(_rf) => Err(Error::NotImplementedError),
    }
}

fn debug_case(df: DebugFrame, prov: Provider) -> Result<()> {
    debug!("enter to debug case frame");
    let data = lacrosse_v3_protocol::decrypt(df.pulses.as_ref()).context(LacrosseError)?;
    debug!(
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
       let s_temp = Sensor::new(id_temp,SensorType::Temperature,"Celcius".into()); 
       (repo.insert_sensor)(&s_temp).context(DatabaseAccessError)?;
    }

    if !(sensor_exist(id_hum, prov)?) {
       let s_hum = Sensor::new(id_hum,SensorType::Humidity,"%".into()); 
       (repo.insert_sensor)(&s_hum).context(DatabaseAccessError)?;
    }

    (repo.insert_sensor_state)(&state_temp).context(DatabaseAccessError)?;
    (repo.insert_sensor_state)(&state_hum).context(DatabaseAccessError)?;


    Ok(())
}

fn sensor_exist(id: SensorId, prov: Provider) -> Result<bool> {
    let prov = prov();
    let sensors = (prov.get_all_sensor)().context(DatabaseAccessError)?;

    let exist = sensors.into_iter().find(|s| s.id == id).is_some();
    Ok(exist)
}
