use crate::schema::*;
use crate::event_hub::models::*;
use crate::event_hub::rules::*;
use diesel::*;
use crate::connection::*;

pub struct EventProvider {
    pub get_all_event           : Box<dyn Fn() -> QueryResult<Vec<Event>>>,
    pub get_event_no_read       : Box<dyn Fn() -> QueryResult<Vec<Event>>>,
    pub insert_event            : Box<dyn Fn(&InsertableEvent) -> QueryResult<Event>>,
    pub insert_rules_result     : Box<dyn Fn(&RulesResult) -> QueryResult<usize>>,
    pub get_last_rules_by_name  : Box<dyn Fn(&str) -> QueryResult<Option<RulesResult>>>,
    pub get_last_event_by_type  : Box<dyn Fn(&EventType) -> QueryResult<Option<Event>>>,
    pub set_event_to_read       : Box<dyn Fn(&i32) -> QueryResult<usize>>
}

impl EventProvider {
    pub fn new() ->  EventProvider {
        EventProvider{
            get_all_event : Box::new(move || {
                
                let locker = DB_POOL.clone();
                let pool = locker.lock().unwrap();
                let conn = pool.get().unwrap();
                event::table.load::<Event>(&conn) }),
            
            get_event_no_read : Box::new(move || {
                let locker = DB_POOL.clone();
                let pool = locker.lock().unwrap();
                let conn = pool.get().unwrap();
                use crate::schema::event::dsl::read_flag;

                event::table.filter(read_flag.eq(false)).load::<Event>(&conn)
            }),

            insert_event : Box::new(move |iev: &InsertableEvent| {
                let locker = DB_POOL.clone();
                let pool = locker.lock().unwrap();
                let conn = pool.get().unwrap();
                use diesel::result::Error;
                use crate::schema::event::dsl::event_id;
                let conn = &conn;
                conn.transaction::<_,Error,_>(|| {
                    diesel::insert_into(event::table).values(iev).execute(conn)?;
                    event::table.order(event_id.desc()).first::<Event>(conn)
                })
            }),

            insert_rules_result : Box::new(move |rr : &RulesResult| {
                let locker = DB_POOL.clone();
                let pool = locker.lock().unwrap();
                let conn = pool.get().unwrap();
                diesel::insert_into(rules_result::table).values(rr).execute(&conn)
            }),

            get_last_rules_by_name : Box::new(move |r_name : &str| {
                let locker = DB_POOL.clone();
                let pool = locker.lock().unwrap();
                let conn = pool.get().unwrap();
                use crate::schema::rules_result::dsl::rule_name;
                use crate::schema::rules_result::dsl::dt_execution;

                rules_result::table.filter(rule_name.eq(r_name)).order(dt_execution.desc()).first::<RulesResult>(&conn).optional()
            }),

            get_last_event_by_type : Box::new(move |e_type : &EventType| {
                let locker = DB_POOL.clone();
                let pool = locker.lock().unwrap();
                let conn = pool.get().unwrap();
                use crate::schema::event::dsl::event_type;
                use crate::schema::event::dsl::dt_occurs;

                event::table.filter(event_type.eq(format!("{}",e_type))).order(dt_occurs.desc()).first::<Event>(&conn).optional()
            }),

            set_event_to_read : Box::new(move |id : &i32| {
                let locker = DB_POOL.clone();
                let pool = locker.lock().unwrap();
                let conn = pool.get().unwrap();
                use crate::schema::event::dsl::event_id;
                use crate::schema::event::dsl::read_flag;
                diesel::update(event::table.filter(event_id.eq(id))).set(read_flag.eq(true)).execute(&conn)
            })
        }
    }
}
















