extern crate diesel;
extern crate diesel_mini;

use self::diesel::prelude::*;
use self::diesel_mini::*;
use self::models::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    insert_measurement(&connection);
    show_measurements(&connection);
}

fn insert_measurement(conn: &MysqlConnection) {
    println!("Input measurements:");
    println!("-------------------");

    let mut floats: Vec<f64> = Vec::with_capacity(3);

    while floats[..].len() < 3 {
        println!("Please write temperature, humidity and pressure");
        println!("(separated by spaces):");
        let mut buffer = String::new();
        let parser = |x: &str| {
            x.parse::<f64>().expect("Aborting! Non-number entered!")
        };
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                floats = buffer
                    .split_whitespace()
                    .take(3)
                    .map(parser)
                    .collect()
            }

            Err(error) => println!("error reading input: {}", error),
        }
    }

    let mut text_input = String::new();
    println!("Please write a comment or leave the field emtpy.");
    stdin().read_line(&mut text_input).unwrap();

    let comment = match text_input[..].trim() {
        "" => None,
        s => Some(s),
    };

    let comeback = create_measurement(
        conn, floats[0], floats[1], floats[2], comment,
    ).unwrap();

    println!("{} row(s) affected!", comeback);
    println!("Thank you for your cooperation!");
    println!("-------------------------------");
}

fn show_measurements(conn: &MysqlConnection) {
    use diesel_mini::schema::measurements::dsl::*;

    let results = measurements
        .load::<Measurement>(conn)
        .expect("Error querying measurements.");

    println!("Displaying {} measurements:", results.len());

    for measurement in results {
        println!(
            "temp: {}°C, humidity: {}%, pressure: {} Pa. {}",
            measurement.temperature,
            measurement.humidity,
            measurement.pressure,
            measurement.comment.unwrap_or_default(),
        );
    }
}
