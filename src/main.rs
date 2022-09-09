pub mod database;
use chrono::DateTime;
use diesel::prelude::*;
#[macro_use]
extern crate rocket;
use database::{
    models::{NewSensorData, Node, SensorData},
    schema::{nodes, sensor_data},
};
use rocket::{
    response::status::{Created, NoContent, NotFound},
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_sync_db_pools::database;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub details: String,
}

#[database("rws")]
pub struct PgConnectionGuard(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PgConnectionGuard::fairing())
        .mount("/", routes![get_node_one, create_data, get_data_some])
}

#[rocket::get("/nodes/<id>")]
async fn get_node_one(
    connection: PgConnectionGuard,
    id: i64,
) -> Result<Json<Node>, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| nodes::table.filter(nodes::id.eq(id)).first(c))
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

//TODO: A more descriptive error if datetime parse fails.
// http://0.0.0.0:5713/data?startts=2022-09-09T11:09:53%2b00:00&endts=2022-09-09T19:15:00.00%2b00:00
#[rocket::get("/data?<startts>&<endts>")]
async fn get_data_some(
    connection: PgConnectionGuard,
    startts: String,
    endts: String,
) -> Result<Json<Vec<SensorData>>, NoContent> {
    let start_time = match DateTime::parse_from_rfc3339(startts.as_str()) {
        Ok(res) => res,
        Err(_err) => {
            println!("Error parsing start ts");
            return Err(NoContent);
        }
    };

    let end_time = match DateTime::parse_from_rfc3339(endts.as_str()) {
        Ok(res) => res,
        Err(_err) => {
            println!("Error parsing end ts");
            return Err(NoContent);
        }
    };

    connection
        .run(move |c| {
            sensor_data::table
                .filter(sensor_data::measured_at.between(start_time, end_time))
                .load(c)
        })
        .await
        .map(Json)
        .map_err(|_e| NoContent)
}

#[rocket::post("/data", data = "<data>")]
async fn create_data(
    connection: PgConnectionGuard,
    data: Json<NewSensorData>,
) -> Result<Created<Json<SensorData>>, Json<ApiError>> {
    let incoming = data.into_inner();
    let result = connection
        .run(move |c| {
            diesel::insert_into(sensor_data::table)
                .values(&incoming)
                .get_result(c)
        })
        .await
        .map(|a| Created::new("/").body(Json(a)))
        .map_err(|e| {
            Json(ApiError {
                details: e.to_string(),
            })
        });
    println!("{:#?}", result);
    result
}
