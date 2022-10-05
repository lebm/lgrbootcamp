use std::fs;
fn main() {
    let first_line = read_first_line("example.txt").unwrap();
    println!("{first_line}");

    let first_line = read_first_line("example2.txt").unwrap();
    println!("{first_line}");
}

fn read_first_line(filename: &str) -> Option<String> {
    // and_then passes the content of Ok to the closure, otherwise returns an error.
    fs::read_to_string(filename).ok().and_then(|s| {
        // lines returns an iterator over the lines ina a string.
        s.lines().next().map(|s| s.to_owned())
    })
}
