pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use self::models::{Measurement, NewMeasurement};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("Environment variable DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_measurement<'a>(
    conn: &MysqlConnection, 
    temperature: f64,
    humidity: f64,
    pressure: f64,
    comment: Option<&'a str>
    ) -> QueryResult<usize> {

    use schema::measurements;

    let new_measurement = NewMeasurement {
        temperature,
        humidity,
        pressure,
        comment,
    };

    diesel::insert_into(measurements::table)
        .values(&new_measurement)
        .execute(conn)
}
