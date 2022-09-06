#[allow(unused_variables)]
fn main() {
    let b1: bool = true;

    let u1: u8 = 1;
    let u2: u16 = 1;
    let u3: u32 = 1;
    let u4: u64 = 1;
    let u5: u128 = 1;

    let s1: i8 = 1;
    let s2: i16 = 1;
    let s3: i32 = 1;
    let s4: i64 = 1;
    let s5: i128 = 1;

    let f1: f32 = 1.0;
    let f2: f64 = 1.0;

    let p1: usize = 1;
    let p2: isize = 1;

    let c1: char = 'c';
    let s1: &str = "hello";
    let s2: String = String::from("hello");

    let a1: [i32; 5] = [1, 2, 3, 4, 5];

    let i1: i32 = a1[4];

    let t1: (i32, i32, i32) = (1, 2, 3);
    let t2: (i32, f64, &str) = (1, 2.0, "3");

    let s1: &str = t2.2;

    // Destructuring a tuble
    let (i1, f1, s1) = t2;

    let unit = ();

    type Age = u8;

    let a1: Age = 57;
}
