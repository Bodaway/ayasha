use crate::sensor::models::*;
use chrono::Duration;
use ::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SensorStatus {
    pub detail: Sensor,
    pub value: f32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct LocationStatus {
    pub location: Location,
    pub sensors: Vec<SensorStatus>,
}

impl LocationStatus {
    fn get_displayable_sensor_value(states: Vec<SensorState>) -> f32 {
        let limit = chrono::Local::now().naive_local() - Duration::minutes(1);
        let valid = states
            .into_iter()
            .filter(|s| s.dt_update > limit)
            .map(|s| s.sensor_value)
            .collect::<Vec<f32>>();
        println!("limit = {}", limit);
        let st_len = valid.len();
        match st_len {
            0 => -99 as f32,
            _ => valid.into_iter().sum::<f32>() / (st_len as f32),
        }
    }

    pub fn get_all_status(
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

#[cfg(test)]
mod TestSensor {

    use crate::sensor::hl_models::*;
    use crate::sensor::models::*;
    #[test]
    fn test_display_ok() {
        let sample = vec![
            SensorState {
                id: 0,
                sensor_id: 0,
                dt_update: chrono::Local::now().naive_local(),
                sensor_value: 12.5,
            },
            SensorState {
                id: 0,
                sensor_id: 0,
                dt_update: chrono::Local::now().naive_local(),
                sensor_value: 12 as f32,
            },
            SensorState {
                id: 0,
                sensor_id: 0,
                dt_update: chrono::Local::now().naive_local(),
                sensor_value: 13 as f32,
            },
        ];
        let result = LocationStatus::get_displayable_sensor_value(sample);
        assert_eq!(result, 12.5)
    }

    #[test]
    fn test_display_old_state() {
        let now = chrono::Local::now().naive_local();
        let old = now - Duration::minutes(1);
        let sample = vec![
            SensorState {
                id: 0,
                sensor_id: 0,
                dt_update: chrono::Local::now().naive_local(),
                sensor_value: 12.5,
            },
            SensorState {
                id: 0,
                sensor_id: 0,
                dt_update: chrono::Local::now().naive_local(),
                sensor_value: 12 as f32,
            },
            SensorState {
                id: 0,
                sensor_id: 0,
                dt_update: chrono::Local::now().naive_local(),
                sensor_value: 13 as f32,
            },
            SensorState {
                id: 0,
                sensor_id: 0,
                dt_update: chrono::Local::now().naive_local() - Duration::minutes(1),
                sensor_value: 31 as f32,
            },
            SensorState {
                id: 0,
                sensor_id: 0,
                dt_update: chrono::Local::now().naive_local() - Duration::minutes(1),
                sensor_value: 31 as f32,
            },
            SensorState {
                id: 0,
                sensor_id: 0,
                dt_update: chrono::Local::now().naive_local() - Duration::minutes(2),
                sensor_value: 31 as f32,
            },
            SensorState {
                id: 0,
                sensor_id: 0,
                dt_update: chrono::Local::now().naive_local() - Duration::minutes(2),
                sensor_value: 31 as f32,
            },
        ];
        let result = LocationStatus::get_displayable_sensor_value(sample);
        assert_eq!(result, 12.5)
    }

    #[test]
    fn test_display_only_old_state() {
        let now = chrono::Local::now().naive_local();
        let old = now - Duration::minutes(1);
        let sample = vec![ 
            SensorState {
                id: 0,
                sensor_id: 0,
                dt_update: chrono::Local::now().naive_local() - Duration::minutes(1),
                sensor_value: 31 as f32,
            },
            SensorState {
                id: 0,
                sensor_id: 0,
                dt_update: chrono::Local::now().naive_local() - Duration::minutes(1),
                sensor_value: 31 as f32,
            },
            SensorState {
                id: 0,
                sensor_id: 0,
                dt_update: chrono::Local::now().naive_local() - Duration::minutes(2),
                sensor_value: 31 as f32,
            },
            SensorState {
                id: 0,
                sensor_id: 0,
                dt_update: chrono::Local::now().naive_local() - Duration::minutes(2),
                sensor_value: 31 as f32,
            },
        ];
        let result = LocationStatus::get_displayable_sensor_value(sample);
        assert_eq!(result, -99 as f32)
    }
}
