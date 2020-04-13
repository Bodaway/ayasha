

pub mod repository;
pub mod models;
pub mod rules;

use crate::event_hub::rules::*;
use crate::event_hub::models::*;
use crate::event_hub::repository::*;

fn nb_day_between_date_and_today(date : &chrono::NaiveDateTime) -> i64 {
    chrono::Local::today().naive_local().signed_duration_since(date.date()).num_days()
}

pub fn get_rules(provide_of_event : &'static dyn Fn () -> EventProvider) -> Vec<Rule> {
    vec![
        Rule {
            rules_name : "OuvrirFenetre".to_string(),
            conditions : Box::new(move || {
                let last : Option<RulesResult> = (provide_of_event().get_last_rules_by_name)("OuvrirFenetre").expect("fail during find last rule OuvrirFenetre");
                match last {
                    None => (true,"Ok".to_string()),
                    Some(last) => (!RulesResult::is_today(last),"Deja executer aujourd hui".to_string())
                }
            }),
            actions : Box::new(||{
                (
                    RulesResult {
                        rule_name : "OuvrirFenetre".to_string(),
                        dt_execution : chrono::Local::now().naive_local(),
                        success : true,
                        details : "".to_string()
                    } ,
                    vec![
                        InsertableEvent::new(EventType::OpenWindows, "OuvrirFenetre".to_string(), None,None,None)
                    ]
                )
            })
        },
        Rule {
            rules_name : "PeriodeSansExposition".to_string(),
            conditions : Box::new(move || {
                let last_rule : Option<RulesResult> = (provide_of_event().get_last_rules_by_name)("PeriodeSansExposition").expect("fail during find last rule PeriodeSansExposition");
                let last_expose : Option<Event> = (provide_of_event().get_last_event_by_type)(&EventType::Covid19ExposedToday).expect("fail during find last covid19 Exposed");
                match (last_rule,last_expose) {
                    (_, None) => (false,"Jamais exposer".to_string()),
                    (None,Some(last)) => {
                        let nb_jours = nb_day_between_date_and_today(&last.dt_occurs); 
                        (nb_jours < 15, format!("nombre de jours depuis event {}", nb_jours))
                    },
                    (Some(last_rule),Some(last)) => {
                        let is_today = RulesResult::is_today(last_rule); 
                        let nb_jours = nb_day_between_date_and_today(&last.dt_occurs);
                        (!is_today & (nb_jours < 15), format!("deja executer aujourdhui vaut {} et nombre de jours depuis event {}",is_today,nb_jours))

                    }
                }
            }),
            actions : Box::new(move ||{

                let last_expose : Option<Event> = (provide_of_event().get_last_event_by_type)(&EventType::Covid19ExposedToday).expect("fail during find last covid19 Exposed");
                let time_wt_exposure: i64 = match last_expose {
                    Some(expose) => chrono::Local::today().naive_local().signed_duration_since(expose.dt_occurs.date()).num_days(),
                    None => panic!("derniere exposition non trouver")
                };

                (
                    RulesResult {
                        rule_name : "PeriodeSansExposition".to_string(),
                        dt_execution : chrono::Local::now().naive_local(),
                        success : true,
                        details : "".to_string()
                    } ,
                    vec![
                        InsertableEvent::new(EventType::Covid19withoutExposure, "PeriodeSansExposition".to_string(), None,Some(time_wt_exposure.to_string()),Some("Temps sans exposition".to_string()))
                    ]
                )
            })
        }
    ]
}

pub fn execute_rules(rules : &Vec<Rule>) -> Vec<(RulesResult,Vec<InsertableEvent>)> {
    rules.iter().map(|r| r.process()).collect()
}