use std::{fs, num::ParseIntError};

fn main() {
    //let i = parse_file("example.txt");
    let i = parse_file2("example.txt");
    match i {
        Ok(i) => println!("{i}"),
        Err(e) => match e {
            ParseFileError::File => {
                println!("Erro em file");
            }
            ParseFileError::Parse(e) => {
                print!("Error parsing number: {e}");
            }
        },
    }
}

// The function returns different type os errors, so the line below is wrong.
//fn parse_file(filename: &str) -> Result<i32, io::Error> {
// One solution, trait objects
// fn parse_file(filename: &str) -> Result<i32, Box<dyn error::Error>> {
// The "?" implicitly converts concrete error types into trait objects
// Simple, but the callee will not know exactly which concrete error was returned at compile time.
// let s = fs::read_to_string(filename)?;
// let i = s.parse()?;
// Ok(i)
// }
//
enum ParseFileError {
    File,
    Parse(ParseIntError),
}

fn parse_file2(filename: &str) -> Result<i32, ParseFileError> {
    #[allow(unused_variables)]
    let s = fs::read_to_string(filename).map_err(|e| ParseFileError::File)?;
    let i = s.trim().parse().map_err(|e| ParseFileError::Parse(e))?;
    Ok(i)
}
