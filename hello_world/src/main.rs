//! Let´s start with the infamous "Hello World!  
//! * "fn" starts a function declaration.  
//! * "main()" is the entry point of the program.  
//! * "println!"" is a macro (ends with "!) that display the values passed as arguments.  
//! * "Hello World!" is the a "str" (string slice), but println! receives a "&str" (reference to a string slice). In practice, "str" and "&str" are called "string slice".  
fn main() {
    println!("Hello, world!");
}
