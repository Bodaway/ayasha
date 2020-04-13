



#[cfg(test)]
mod TestSensor {
    
use crate::sensor::models::*;
use crate::sensor::repository::*;
use crate::connection::*;   
use crate::schema::*;
use diesel;
use diesel::prelude::*;
use std::{thread, time};

    #[test]
    fn test_insert_sensor () {

        let conn = establish();
        diesel::delete(sensor::table).execute(&conn);
        diesel::delete(sensor_state::table).execute(&conn);



        let repo = SensorProvider::new(&establish);
        let i_sensor = InsertableSensor::new("Sensor0","Chambre de marie",SensorType::Temperature,"C");
        let result_sensor_0 = (repo.insert_sensor)(i_sensor);
        let sensor_0 = match result_sensor_0 {
            Ok(sensor) => sensor,
            Err(ex) => panic!("Problem inserting data : {:?}",ex)
        };
        let s0_1 = sensor_0.create_state(25.0);
        match (repo.insert_sensor_state)(&s0_1) {
            Err(ex) => panic!("Problem inserting data : {:?}",ex),
            Ok(u) => u
        };
        thread::sleep(time::Duration::from_millis(1000));
        let s0_2 = sensor_0.create_state(28.2);
        match (repo.insert_sensor_state)(&s0_2) {
            Err(ex) => panic!("Problem inserting data : {:?}",ex),
            Ok(u) => u
        };
        
        
         
        let i_sensor_1 = InsertableSensor::new("Sensor1","Chambre de parent",SensorType::Temperature,"C");
        let result_sensor_1 = (repo.insert_sensor)(i_sensor_1);
        let sensor_1 = match result_sensor_1 {
            Ok(sensor) => sensor,
            Err(ex) => panic!(ex)
        };
        let s1_1 = sensor_1.create_state(30.5);
        match (repo.insert_sensor_state)(&s1_1) {
            Err(ex) => panic!("Problem inserting data : {:?}",ex),
            Ok(u) => u
        };
        thread::sleep(time::Duration::from_millis(1000));
        let s1_2 = sensor_1.create_state(32.7);
        match (repo.insert_sensor_state)(&s1_2) {
            Err(ex) => panic!("Problem inserting data : {:?}",ex),
            Ok(u) => u
        };


        let result_all_sensor = (repo.get_all_sensor)();
        let all_sensor = match result_all_sensor {
            Ok(all_sensor) => all_sensor,
            Err(ex) => panic!(ex)
        };

        assert_eq!(all_sensor[0].name,"Sensor0");
        assert_eq!(all_sensor[1].name,"Sensor1");


        let result_all_sensor_state = (repo.get_all_sensor_state)();
        let all_sensor_state = match result_all_sensor_state {
            Ok(all_sensor_state) => all_sensor_state,
            Err(ex) => panic!(ex)
        };
        assert_eq!(all_sensor_state[0].sensor_id,all_sensor[0].sensor_id);
        assert_eq!(all_sensor_state[0].sensor_value,25.0);
    }
}
