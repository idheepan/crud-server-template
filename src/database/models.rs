use super::schema::sensor_data;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = sensor_data)]
pub struct NewSensorData {
    pub measured_at: DateTime<Utc>,
    pub sensor_id: i64,
    pub values: serde_json::Value,
}

#[derive(Queryable, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SensorData {
    pub id: i64,
    pub inserted_at: DateTime<Utc>,
    pub measured_at: DateTime<Utc>,
    pub sensor_id: i64,
    pub values: serde_json::Value,
}

#[derive(Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Node {
    pub id: i64,
    pub description: Option<String>,
}
