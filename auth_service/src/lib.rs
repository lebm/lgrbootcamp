#![allow(dead_code, unused_variables)]
// Brings to the scope, not importing.
// It is visible to the scope but implemented elsewhere
use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}

mod database {

    pub enum Status {
        Connected,
        Interrupted,
    }

    pub fn connect_to_database() -> Status {
        Status::Connected
    }

    pub fn get_user() {}
}

mod auth_utils {

    pub fn login(creds: models::Credentials) {
        crate::database::get_user();
    }

    fn logout(creds: models::Credentials) {}

    pub mod models {
        pub struct Credentials {
            username: String,
            password: String,
        }
    }
}
