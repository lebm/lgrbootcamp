fn main() {
    // Its is better to watch the videos.
    // The compiler checks that, when a value is used, it is still valid, it was not dropped.
    // It also checks references. A reference lifetime must be contained in the lifetime of the referenced value.
    // The compiler is able to detect "no lexical lifetime".
    // If a reference is not used after a point in the program, the compiler assumes its lifetime ended.
    println!("Hello, world!");
}
