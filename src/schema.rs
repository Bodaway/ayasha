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
    sensor (sensor_id) {
        sensor_id -> Integer,
        name -> Text,
        location -> Text,
        sensor_type -> Text,
        unit -> Text,
    }
}

table! {
    sensor_state (sensor_id, dt_update) {
        sensor_id -> Integer,
        dt_update -> Timestamp,
        sensor_value -> Float,
    }
}

allow_tables_to_appear_in_same_query!(
    event,
    raw_frame_info,
    rules_result,
    sensor,
    sensor_state,
);
