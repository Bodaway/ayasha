use crate::sensor::models::*;
use chrono::Duration;

pub struct SensorStatus {
    pub detail: Sensor,
    pub value: String,
}
pub struct LocationStatus {
    pub location: Location,
    pub sensors: Vec<SensorStatus>,
}

impl LocationStatus {
    fn get_displayable_sensor_value(states: Vec<SensorState>) -> String {
        let limit = chrono::Local::now().naive_local() - Duration::minutes(1);
        let st_len = states.len() as f32;
        let value = states
            .into_iter()
            .filter(|s| s.dt_update > limit)
            .map(|s| s.sensor_value)
            .sum::<f32>()
            / st_len;

        format!("{}", value)
    }

    fn get_all_status(
        data: Vec<(Location, Vec<(Sensor, Vec<SensorState>)>)>,
    ) -> Vec<LocationStatus> {
        data.into_iter()
            .map(|(l, s)| LocationStatus {
                location: l,
                sensors: s
                    .into_iter()
                    .map(|(se, st)| SensorStatus {
                        detail: se,
                        value: LocationStatus::get_displayable_sensor_value(st),
                    })
                    .collect(),
            })
            .collect()

    }
}
