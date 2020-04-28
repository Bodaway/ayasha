use diesel;
use diesel::prelude::*;
use crate::schema::*;

use crate::sensor::models::*;



pub struct SensorProvider {
    pub get_all_sensor_state    : Box<dyn Fn() -> QueryResult<Vec<SensorState>>>,
    pub get_last_sensor_state   : Box<dyn Fn() -> QueryResult<Vec<SensorState>>>,
    pub get_all_sensor          : Box<dyn Fn() -> QueryResult<Vec<Sensor>>>,
    pub insert_sensor           : Box<dyn Fn(&Sensor) -> QueryResult<usize>>,
    pub insert_location         : Box<dyn Fn(InsertableLocation) -> QueryResult<Location>>,
    pub insert_sensor_state     : Box<dyn Fn(&SensorState) -> QueryResult<usize>>,
}


impl SensorProvider {
    pub fn new(connection_provider : &'static dyn Fn() -> SqliteConnection) -> SensorProvider {
        SensorProvider {
            get_all_sensor_state : Box::new(move || sensor_state::table.load::<SensorState>(&(connection_provider()))),

            get_last_sensor_state : Box::new(move || {
                diesel::dsl::sql_query("select * from 
                            (
	                        select sensor_id, sensor_value,dt_update
	                        from sensor_state
	                        order by dt_update desc
                            ) a group by sensor_id;"
                            ).load::<SensorState>(&(connection_provider()))
            }),

            get_all_sensor : Box::new(move || sensor::table.load::<Sensor>(&(connection_provider()))),

            insert_sensor : Box::new(move |isensor: &Sensor| {
                diesel::insert_into(sensor::table).values(isensor).execute(&connection_provider())
            }),

            insert_location : Box::new(move |il: InsertableLocation| {
                use diesel::result::Error;
                use crate::schema::location::dsl::id;
                let conn = connection_provider();
                conn.transaction::<_,Error,_>(|| {
                    diesel::insert_into(location::table).values(&il).execute(&conn)?;
                    location::table.order(id.desc()).first::<Location>(&conn)
                })
            }),

            insert_sensor_state : Box::new(move |ss : &SensorState| {
                diesel::insert_into(sensor_state::table).values(ss).execute(&connection_provider())})
        }
    }
}
