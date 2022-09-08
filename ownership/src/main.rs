#[allow(unused_variables)]
fn main() {
    // s1 has a metadata on the stack that owns a pointer to the string on the heap.
    let s1 = String::from("Rust");
    println!("s1 is: {s1}");
    f();
    primitives();

    let mut s1 = String::from("Rust");
    let s2 = s1;
    // Error: s1 was moved to s2. It is not initialized anymore.
    //println!("s1 is: {s1}");
    println!("s2 is: {s2}");

    // clone method can create a copy of a value.
    // There will be two "Rust" strings on the heap. One owned be s1 and the other owned by s2.
    s1 = s2.clone();
    println!("s1 clone is: {s1}");

    // When the function ends, s1 and s2 will be dropped from the stack, but first they will drop the string from the heap.
}

#[allow(unused_variables)]
fn f() {
    {
        let s1 = String::from("Rust");
        println!("s1 is {s1} in f()");
        // s1 is dropped from the stack here, ate the end of s1 scope.
    }

    // Error: s1 was dropped in the inner scope above.
    //println!("s1 is: {s1}");
}

fn primitives() {
    // for primitives the rules are similar to rules on other languages.
    // Primitive types, allocated on the stack, implement Copy trait. They are copied/cloned, not moved.
    // For types allocated on the stack, copy is as efficient as move, and uses the same memory, so they are copied.
    let x = 10;
    let y = x;
    println!("x y: {x} {y}");
}
