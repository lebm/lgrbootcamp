#![allow(dead_code, unused_variables)]
use rand::prelude::*;

// The database definition are in database.rs file
mod auth_utils;
mod database;

// Brings to the scope, not importing.
// It is visible to the scope but implemented elsewhere
// "pub use" reexports the imported itens
pub use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    let timeout = thread_rng().gen_range(100..500);
    println!("The timeout is {timeout}");
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}
