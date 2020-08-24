extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::{env, process};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

use crate::model::Photo;

mod model;
mod schema;

fn main() {
    if let Err(error) = find_all() {
        println!("{:?}", error);
        process::exit(1);
    }
}

#[test]
fn test_update() {
    let conn = establish_connection();

    match update(&conn) {
        Err(err) => {
            println!("{:?}", err);
            process::exit(1);
        }
        Ok(size) => println!("{:?}", size),
    }
}

fn update(conn: &PgConnection) -> QueryResult<usize> {
    use self::schema::photos::dsl::*;
    diesel::insert_into(photos)
        .values(&vec![
            (id.eq(1111), name.eq("aaaa"), description.eq("AAAAAAAA")),
            (id.eq(2222), name.eq("bbbb"), description.eq("BBBBBBB")),
        ])
        .execute(conn)
}

fn find_all() -> Result<(), Box<diesel::result::Error>> {
    use self::schema::photos::dsl::*;
    let connection = establish_connection();
    let results = photos
        .filter(description.eq("AAA"))
        .limit(5)
        .load::<Photo>(&connection)?;

    println!("Displaying {} posts", results.len());
    for photo in results {
        println!("{}", photo.name);
        println!("----------\n");
        println!("{}", photo.description.unwrap());
    }

    Ok(())
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
