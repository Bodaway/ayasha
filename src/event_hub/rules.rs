use chrono::NaiveDateTime;
use crate::event_hub::models::InsertableEvent;
use crate::schema::*;


#[derive(Queryable,Insertable)]
#[table_name = "rules_result"]
pub struct RulesResult {
    pub rule_name : String,
    pub dt_execution : NaiveDateTime,
    pub success : bool,
    pub details : String
}

impl RulesResult {
    pub fn is_today(rr : RulesResult) -> bool {
        rr.dt_execution.date() == chrono::Local::today().naive_local()
    }
}


pub struct Rule {
    pub rules_name : String,
    pub conditions : Box<dyn Fn() -> (bool,String)>,
    pub actions : Box<dyn Fn() -> (RulesResult,Vec<InsertableEvent>)>
}

impl Rule {
    pub fn process(&self) -> (RulesResult,Vec<InsertableEvent>) {
        let (result,info) = (self.conditions)();
            if result {
                (self.actions)()
            }
            else {
                (RulesResult{
                    rule_name : self.rules_name.clone(),
                    dt_execution : chrono::Local::now().naive_local(),
                    success : true,
                    details : info
                }, Vec::new())
            }

    }

}
