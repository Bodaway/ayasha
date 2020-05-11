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
                sensor_state::table.load::<SensorState>(&DB_POOL.get().unwrap())
            }),

            get_last_sensor_state: Box::new(move || {
                diesel::dsl::sql_query(
                    "select * from 
                            (
	                        select id,sensor_id, sensor_value,dt_update
	                        from sensor_state
	                        order by dt_update desc
                            ) a group by sensor_id;",
                )
                .load::<SensorState>(&DB_POOL.get().unwrap())
            }),

            get_all_sensor: Box::new(move || {
                sensor::table.load::<Sensor>(&DB_POOL.get().unwrap())
            }),

            insert_sensor: Box::new(move |isensor: &Sensor| {
                diesel::insert_into(sensor::table)
                    .values(isensor)
                    .execute(&DB_POOL.get().unwrap())
            }),

            update_sensor_location: Box::new(move |sid: SensorId, lid: LocationId| {
                use crate::schema::sensor::id;
                use crate::schema::sensor::location_id;
                diesel::update(sensor::table.filter(id.eq(sid)))
                    .set(location_id.eq(lid))
                    .execute(&DB_POOL.get().unwrap())
            }),
            insert_location: Box::new(move |il: InsertableLocation| {
                use crate::schema::location::dsl::id;
                use diesel::result::Error;
                let conn = &DB_POOL.get().unwrap();
                conn.transaction::<_, Error, _>(|| {
                    diesel::insert_into(location::table)
                        .values(&il)
                        .execute(conn)?;
                    location::table.order(id.desc()).first::<Location>(conn)
                })
            }),

            insert_sensor_state: Box::new(move |ss: &InsertableSensorState| {
                diesel::insert_into(sensor_state::table)
                    .values(ss)
                    .execute(&DB_POOL.get().unwrap())
            }),

            get_all_location_status: Box::new(move || {
                let conn = &DB_POOL.get().unwrap();
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
