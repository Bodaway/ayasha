table! {
    event (event_id) {
        event_id -> Integer,
        event_type -> Text,
        sensor_source_id -> Nullable<Integer>,
        rule_name -> Text,
        event_value -> Nullable<Text>,
        event_value_definition -> Nullable<Text>,
        dt_occurs -> Timestamp,
        read_flag -> Bool,
    }
}

table! {
    location (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    raw_frame_info (frame_id) {
        frame_id -> Integer,
        raw_data -> Text,
        dt_occurs -> Timestamp,
    }
}

table! {
    rules_result (rule_name, dt_execution) {
        rule_name -> Text,
        dt_execution -> Timestamp,
        success -> Bool,
        details -> Text,
    }
}

table! {
    sensor (id) {
        id -> Integer,
        location_id -> Nullable<Integer>,
        sensor_type -> Text,
        unit -> Text,
    }
}

table! {
    sensor_state (id) {
        id -> Integer,
        sensor_id -> Integer,
        dt_update -> Timestamp,
        sensor_value -> Float,
    }
}

joinable!(sensor -> location (location_id));
joinable!(sensor_state -> sensor (sensor_id));

allow_tables_to_appear_in_same_query!(
    event,
    location,
    raw_frame_info,
    rules_result,
    sensor,
    sensor_state,
);
