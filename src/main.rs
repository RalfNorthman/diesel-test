extern crate diesel;
extern crate diesel_mini;

use self::diesel_mini::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use diesel_mini::schema::measurements::dsl::*;

    let connection = establish_connection();

    let results = measurements
        .limit(10)
        .load::<Measurement>(&connection)
        .expect("Error querying measurements.");

    println!("Displaying {} measurements:", results.len());

    for measurement in results {
        println!("{:?}", measurement);
    }
}
