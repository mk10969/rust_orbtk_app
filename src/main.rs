extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let env_properties = env::var("DATABASE_URL");

    let oooo = env_properties.expect("no property");

    println!("{:?}", "aaaaaaaaaaaaaa");
    println!("{:?}", oooo);
}
