use auth_service_file_modules2::Credentials;

#[allow(unused_variables)]
fn main() {
    let creds = Credentials {
        username: "bravo".to_owned(),
        password: "password".to_owned(),
    };

    auth_service_file_modules2::authenticate(creds);
}
