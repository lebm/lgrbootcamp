use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    #[allow(unused_variables)]
    let contents = read_file("example.txt").expect("Could not read file");
    println!("{contents}");
    let contents = read_file("example2.txt").expect("Could not read example2.txt");
    println!("{contents}");

    let name = User {
        firstname: "Luis".to_owned(),
        lastname: "Bravo".to_owned(),
    };

    let initials = get_initials(name).unwrap();
    println!("{initials}");

    let name = User {
        firstname: "".to_owned(),
        lastname: "".to_owned(),
    };

    let initials = get_initials(name).unwrap();
    println!("{initials}");
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    // PROPAGATIONM THE ERROR WITH "?"
    // The "?" operator returns the value for Ok or Some variants, or returns an error or None for the caller.
    // It can replace the "match { Ok/Same => .., None/Err => return ..}" boilerplate.
    // It can be used only in functions returning Result or Option.
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

struct User {
    firstname: String,
    lastname: String,
}

fn get_initials(user: User) -> Option<String> {
    // chars returns an iterator that returns Some<char> for each character in the string and None when finished.
    // if the first or last name is empty, chars returns None, so "?" returns None to the caller.
    let first_initial = user.firstname.chars().next()?;
    let last_initial = user.lastname.chars().next()?;
    // If it reaches this point, there are initials to be returned.
    // "format!" macros is like "println!", but instead of printing, it creates a String.
    // Finaly, "Some" creates the value to be returned with the string.
    Some(format!("{first_initial}.{last_initial}."))
}
