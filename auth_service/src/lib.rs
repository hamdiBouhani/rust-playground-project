#![allow(dead_code, unused_variables)]
mod auth_utils;
mod database;

pub use auth_utils::models::Credentials; //briging the Credentials to scope and reexporting it
use database::Status;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        println!("Connected to database");
    } else {
        println!("Could not connect to database");
    }
}
