// @generated automatically by Diesel CLI.

diesel::table! {
    nodes (id) {
        id -> Int8,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    sensor_data (id) {
        id -> Int8,
        inserted_at -> Timestamptz,
        measured_at -> Timestamptz,
        sensor_id -> Int8,
        values -> Jsonb,
    }
}

diesel::table! {
    sensors (id) {
        id -> Int8,
        created_at -> Timestamptz,
        connected_to -> Int8,
        sensor_type -> Text,
    }
}

diesel::joinable!(sensor_data -> sensors (sensor_id));
diesel::joinable!(sensors -> nodes (connected_to));

diesel::allow_tables_to_appear_in_same_query!(
    nodes,
    sensor_data,
    sensors,
);
