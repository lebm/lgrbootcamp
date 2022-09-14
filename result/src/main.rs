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
    let query = format!("GET username FROM users WHERE id={user_id}");
    let db_result = query_db(query);
    // ok method converts Result into Option and vice versa.
    db_result.ok()
}

fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("query is empty"))
    } else {
        Ok(String::from("Ferris"))
    }
}
