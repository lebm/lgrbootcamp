#[allow(unused_variables)]
fn main() {
    // https://unicode-table.com/en/1F980/
    let s1: &str = "Olá mundo!🦀";
    let s2: &str = "Olá mundo!\u{1F980}";
    println!("{s1} {s2}");

    let s3 = String::from("Olá mundo!");
    let s4 = "Olá mundo!".to_string();
    let s4 = "Olá mundo!".to_owned();
    // strings slice referencing s4
    let s5 = &s4[..];
    println!("{s3} {s4} {s5}");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.replace_range(.., "baz");
    println!("{s}");

    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = s1 + &s2;
    // Erro: "+" moves s1 to s3 and appends s2
    //println!("{s1}");
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{s}");
    println!("{s1} {s2} {s3}");

    let s1 = ["first", "second"].concat();
    let s2 = format!("{}{}", "first", "second");
    let s3 = concat!("first", "second");
    println!("{s1} {s2} {s3}");

    let crabs = "🦀🦀🦀🦀🦀";
    let first_crab = &crabs[0..4];
    println!("{crabs} {first_crab}");
    // Panics! 0..3 are inside the first crab! Each crab is 4 bytes long.
    //println!("{}", &crabs[0..3]);

    for c in crabs.bytes() {
        println!("{}", c);
    }

    for c in crabs.chars() {
        println!("{}", c);
    }

    let a = my_string("Hello");
    let a = my_string(&"World".to_string());
}

fn my_string(s: &str) -> String {
    s.to_string()
}
