use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    value: T,
}

impl<T> MySmartPointer<T> {
    fn new(value: T) -> Self {
        MySmartPointer { value }
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let s = MySmartPointer::new(Box::new("Hello world!".to_owned()));

    // Deref coercion "converts" one reference to another.
    // Works on types that implement Deref and DerefMut traits.
    // &MySmartPointer -> &Box -> &String -> &str
    print(&s);

    let _s0 = &s;
    let _s1 = &(*s);
    let _s2 = &(**s);
    let _s3 = &(***s);
}

fn print(s: &str) {
    println!("{s}");
}
