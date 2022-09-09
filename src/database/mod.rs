pub(crate) mod models;
pub(crate) mod schema;

use chrono::Utc;
use diesel::prelude::*;
// use dotenvy::dotenv;
// use std::env;

use crate::database::models::NewSensorData;

use self::models::SensorData;

pub fn create_data_entry(
    conn: &mut PgConnection,
    measured_at: chrono::DateTime<Utc>,
    sensor_id: i64,
    values: serde_json::Value,
) -> SensorData {
    use schema::sensor_data;

    let new_data = NewSensorData {
        measured_at,
        sensor_id,
        values,
    };

    diesel::insert_into(sensor_data::table)
        .values(&new_data)
        .get_result(conn)
        .expect("Error saving new post")
}
