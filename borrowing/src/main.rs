#[allow(unused_variables)]
fn main() {
    let s1 = String::from("Rust"); // heapallocated string
    print_string(s1);
    // Invalid. s1 was moved to print_string
    // println!("s1 is : {s1}");

    let s2 = String::from("Ferris"); // heapallocated string
    let r2 = &s2;

    // wrong. print_string expects String, but received &String
    //print_string(r2);

    print_string_ref(r2);
    print_string_ref(&s2);

    let mut s3 = String::from("Bravo");
    let r3 = &mut s3;
    add_to_string(r3);

    // Error:
    // println! borrows shared reference to string ...
    //println!("{s3}");
    // ... but a mutable reference already exists.
    // When there is a mutable reference, its the only reference allowed.
    //println!("{r3}");

    // Ok
    // The lifetime of r3 ends after this println! (non lexical lifetime)
    println!("{r3}");
    // At this point, r3 (mutable reference) is not used anymore, so the compiler allows a shared reference.
    println!("{s3}");

    println!("{}", generate_string());
}

fn print_string(p1: String) {
    println!("{p1}");
}

fn print_string_ref(p1: &String) {
    println!("{p1}");
}

fn add_to_string(p1: &mut String) {
    // rust does automatic dereference. "p1." is equivalen to "(*p1)."
    p1.push_str(" is awesome");
}

// Dangling reference example
// "s" is dropped at the end of the function, so "&s" becames invalid (dangling reference), and the compiler catches it.
// The correct implementation just returns the String
//fn generate_string() -> &String {
//    let s = String::from("Rust");
//    &s
//}
fn generate_string() -> String {
    String::from("Rust")
}
