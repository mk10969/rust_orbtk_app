extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

use crate::model::Photo;

mod model;
mod schema;

fn main() {
    use self::schema::photos::dsl::*;
    let connection = establish_connection();
    let results = photos
        .filter(description.eq("AAA"))
        .limit(5)
        .load::<Photo>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for photo in results {
        println!("{}", photo.name);
        println!("----------\n");
        println!("{}", photo.description.unwrap());
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
