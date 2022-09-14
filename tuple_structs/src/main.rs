#[allow(unused_variables)]
fn main() {
    // tuples
    let rgb_color = (255, 106, 0);
    let cmyk_color = (0, 58, 100, 0);

    // tuples structs
    // More type information/meaning. Allows compiler to check types.
    #[derive(Debug)]
    struct RGB(i32, i32, i32);
    #[derive(Debug)]
    struct CMYK(i32, i32, i32, i32);

    let color1 = RGB(255, 50, 0);
    let color2 = CMYK(0, 58, 100, 0);

    println!("{:?}", color1);
    println!("{:?}", color2);

    // unit struct.
    // Used to implement traits under certain conditions.
    #[allow(dead_code)]
    struct MyStruct;
}
