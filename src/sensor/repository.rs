use crate::schema::*;
use diesel;
use diesel::prelude::*;
use crate::connection::*;

use crate::sensor::models::*;

pub struct SensorProvider {
    pub get_all_sensor_state: Box<dyn Fn() -> QueryResult<Vec<SensorState>>>,
    pub get_last_sensor_state: Box<dyn Fn() -> QueryResult<Vec<SensorState>>>,
    pub get_all_sensor: Box<dyn Fn() -> QueryResult<Vec<Sensor>>>,
    pub insert_sensor: Box<dyn Fn(&Sensor) -> QueryResult<usize>>,
    pub update_sensor_location: Box<dyn Fn(SensorId, LocationId) -> QueryResult<usize>>,
    pub insert_location: Box<dyn Fn(InsertableLocation) -> QueryResult<Location>>,
    pub insert_sensor_state: Box<dyn Fn(&InsertableSensorState) -> QueryResult<usize>>,
    pub get_all_location_status:
        Box<dyn Fn() -> QueryResult<Vec<(Location, Vec<(Sensor, Vec<SensorState>)>)>>>,
}

impl SensorProvider {
    pub fn new() -> SensorProvider {
        SensorProvider {
            get_all_sensor_state: Box::new(move || {
                let locker = DB_POOL.clone();
                let pool = locker.lock().unwrap();
                let conn = pool.get().unwrap();
                sensor_state::table.load::<SensorState>(&conn)
            }),

            get_last_sensor_state: Box::new(move || {
                let locker = DB_POOL.clone();
                let pool = locker.lock().unwrap();
                let conn = pool.get().unwrap();
                diesel::dsl::sql_query(
                    "select * from 
                            (
	                        select id,sensor_id, sensor_value,dt_update
	                        from sensor_state
	                        order by dt_update desc
                            ) a group by sensor_id;",
                )
                .load::<SensorState>(&conn)
            }),

            get_all_sensor: Box::new(move || {
                let locker = DB_POOL.clone();
                let pool = locker.lock().unwrap();
                let conn = pool.get().unwrap();
                sensor::table.load::<Sensor>(&conn)
            }),

            insert_sensor: Box::new(move |isensor: &Sensor| {
                let locker = DB_POOL.clone();
                let pool = locker.lock().unwrap();
                let conn = pool.get().unwrap();
                diesel::insert_into(sensor::table)
                    .values(isensor)
                    .execute(&conn)
            }),

            update_sensor_location: Box::new(move |sid: SensorId, lid: LocationId| {
                let locker = DB_POOL.clone();
                let pool = locker.lock().unwrap();
                let conn = pool.get().unwrap();
                use crate::schema::sensor::id;
                use crate::schema::sensor::location_id;
                diesel::update(sensor::table.filter(id.eq(sid)))
                    .set(location_id.eq(lid))
                    .execute(&conn)
            }),
            insert_location: Box::new(move |il: InsertableLocation| {
                let locker = DB_POOL.clone();
                let pool = locker.lock().unwrap();
                let conn = pool.get().unwrap();
                use crate::schema::location::dsl::id;
                use diesel::result::Error;
                let conn = &conn;
                conn.transaction::<_, Error, _>(|| {
                    diesel::insert_into(location::table)
                        .values(&il)
                        .execute(conn)?;
                    location::table.order(id.desc()).first::<Location>(conn)
                })
            }),

            insert_sensor_state: Box::new(move |ss: &InsertableSensorState| {
                let locker = DB_POOL.clone();
                let pool = locker.lock().unwrap();
                let conn = pool.get().unwrap();
                diesel::insert_into(sensor_state::table)
                    .values(ss)
                    .execute(&conn)
            }),

            get_all_location_status: Box::new(move || {
                let locker = DB_POOL.clone();
                let pool = locker.lock().unwrap();
                let conn = pool.get().unwrap();
                let conn = &conn;
                let locs = location::table.load::<Location>(conn)?;
                let sensors = Sensor::belonging_to(&locs).load::<Sensor>(conn)?;
                let states = SensorState::belonging_to(&sensors).load::<SensorState>(conn)?;

                let gp_states = states.grouped_by(&sensors);
                let sensors_st = sensors.into_iter().zip(gp_states).grouped_by(&locs);

                let result: Vec<(Location, Vec<(Sensor, Vec<SensorState>)>)> =
                    locs.into_iter().zip(sensors_st).collect();

                Ok(result)
            }),
        }
    }
}
