
pub fn traitement_recurent() {
    loop {
        info!("Debut du traitement recurent");
        //Phase 1 : Traitement des signaux
        //Phase de reception des signaux RF (si requis ici)
        //Phase de transformation des signaux RF en object
        //Enregistrement des objects en BD
        //Phase 2 : Traitement des Evenements

        
        use std::{thread, time};

        use crate::connection;
        use crate::event_hub::models::*;
        use crate::event_hub::repository::*;
        use crate::event_hub::*;
        let get_repo = &||EventProvider::new();

        let rules_result = execute_rules(&get_rules(get_repo));

        let event_throwed: Vec<Event> = rules_result
            .iter()
            .map(|(rr, v_iev)| {
                (get_repo().insert_rules_result)(rr).expect("rules result insertion fail");
                v_iev
                    .iter()
                    .map(|iev| (get_repo().insert_event)(iev).expect("event insertion fail"))
                    .collect::<Vec<Event>>()
            })
            .flatten().collect::<Vec<Event>>();

        let notif_data = event_throwed.into_iter().map(|e| e.get_notification_data());
        for notif in notif_data {
            let (title,message) = notif;
            use azure_notificationhub_sender::notification_hub::*;
            
            let body = ["{ \"data\": {\"title\": \"",&*title,"\",\"message\":\"",&*message,"\"}}"].concat();
            debug!("body = {}",body);

            let hub =  NotificationHub::new(&*get_dot_env_var("NOTIFICATION_HUBNAME"), &*get_dot_env_var("NOTIFICATION_CONNECTION_STRING"))
                .parse().unwrap().send_gcm(body);
            thread::sleep(time::Duration::from_secs(10));
        }

        info!("Fin du traitement recurent");
        thread::sleep(time::Duration::from_secs(get_process_freq_dot_env()));
    }
}
fn get_dot_env_var(variable_name : &str) -> String {
    use dotenv::dotenv;
    use std::env;
    
    dotenv().ok();
    env::var(variable_name)
    .expect(&*format!("variable {} must be set",variable_name))

}

fn get_process_freq_dot_env() -> u64 {
    use dotenv::dotenv;
    use std::env;
    
    dotenv().ok();
    env::var("PROCESS_FREQ")
    .expect("PROCESS_FREQ must be set").parse::<u64>().expect("PROCESS_FREQ parse error")
}