use std::fs::File;

fn main() {
    #[allow(unused_variables)]
    // The Result type has many methods.
    let file = File::open("example.txt");

    // Use match to extract file handle or err from Result
    // match is an expression, so you can return its value and use it as an RHS.
    // Resutl is an enum, so match must treat all variants, Ok and Err in this case.
    // The match arms can destruct the result and extacts the value "inside" them.
    #[allow(unused_variables)]
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            panic!("Failed to open file: {:?}", error)
        }
    };

    // Alternatively, you can use unwrap
    // unwrap returns the value insde Ok or panics in case of error.
    // useful for testing purposes.
    // The RHS type will be the type embedded in the Ok result.
    #[allow(unused_variables)]
    let file2 = File::open("example.txt").unwrap();

    // There is also expect.
    // It is like unwrap, but allows you to pass a message.
    #[allow(unused_variables)]
    let file2 = File::open("example.txt").expect("Fail to open example.txt");
}

// Example of a function returning Result
#[allow(dead_code)]
fn get_user_id(username: &str) -> Result<i32, String> {
    if username.is_empty() {
        Err("Username can not be empty".to_owned())
    } else {
        Ok(1)
    }
}
