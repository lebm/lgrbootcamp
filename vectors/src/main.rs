#[allow(unused_variables)]
fn main() {
    #[allow(unused_variables)]
    let v1: Vec<String> = Vec::new();

    let mut v2 = Vec::new();
    v2.push(String::from("One"));
    v2.push(String::from("Two"));
    v2.push(String::from("Three"));

    let v3 = vec![1, 2, 3];

    let s = &v2[0];
    // let s v2.remove(0);

    let s = v2.get(0);

    if let Some(e) = s {
        println!("{e}");
    }

    // mutable borrow of v2 allows to mutate it without copying taking ownership
    // s will mutable borrow each of the elements v2 elements in sequence
    for s in &mut v2 {
        s.push_str("!");
    }

    for s in &v2 {
        println!("{s}");
    }

    // Don't need to annotate the type. The compiler will infer from the loop below.
    let mut v3 = vec![];

    // v2 will be consumed by the loop
    for s in v2 {
        v3.push(s);
    }

    // Erro. v2 is now unitialized
    //let i = v2.get(0);
}
