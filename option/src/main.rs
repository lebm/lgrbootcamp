fn main() {
    let username = get_username(1);
    match username {
        Some(name) => println!("{name}"),
        None => {}
    }

    let username = get_username(1);
    // username is Option<String>, so if it matches Some(name), match will extract name from username.
    if let Some(name) = username {
        println!("{name}");
    }
}

fn get_username(user_id: i32) -> Option<String> {
    let db_result = String::from("Ferris");
    if user_id == 1 {
        Some(db_result)
    } else {
        None
    }
}
