fn main() {
    #[allow(unused_variables)]
    let v1: Vec<String> = Vec::new();

    let mut v2 = Vec::new();
    v2.push(String::from("One"));
    v2.push(String::from("Two"));
    v2.push(String::from("Three"));

    let v2 = vec![1, 2, 3];

    let s = &v2[0];
}
