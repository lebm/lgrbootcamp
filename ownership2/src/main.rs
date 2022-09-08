#[allow(unused_variables)]
fn main() {
    let s1 = String::from("Rust");
    print_string(s1);

    // Error: s1 was moved to print_string ans is not initialized.
    //println!("{s1}");

    // First solution. Cloning.
    // Its not efficient.
    let s1 = String::from("Rust");
    print_string(s1.clone());
    println!("{s1}");

    let s3 = generate_string();
    println!("{s3}");

    let s4 = add_to_string(s3);
    println!("{s4}");

    // Primitive type are not moved, but cloned/copied to functions.
    let x = 10;
    let y = x;
    print_integer(x);
}

fn print_string(p1: String) {
    println!("In print_string: {p1}");
}

fn generate_string() -> String {
    // The expression value will be moved to the return value
    // As this is the return value, it must be am expression, so it can not end with ";"
    String::from("Ferris")
}

fn add_to_string(mut p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}

fn print_integer(i: i32) {
    println!("In print_integer: {i}");
}
