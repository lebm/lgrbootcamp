pub mod models;

pub fn login(creds: models::Credentials) {
    crate::database::get_user();
}

fn logout(creds: models::Credentials) {}
