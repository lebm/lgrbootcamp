#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    // creation
    let a: i16 = 5;

    // mutability
    let mut b = 5;
    b = 10;

    // shadowing
    let c = 10;
    let rc = &c;
    let c = "string";

    // The first "c" still exists, its is only shadowed by the second "c".
    // The rc reference can still access the first "c".
    println!("{} {}", c, rc);

    // scope
    let d = 10;
    {
        let d = "teste";
        println!("{}", d);
    }
    println!("{}", d);
}
