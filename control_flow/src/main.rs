#[allow(unused_variables)]
fn main() {
    let a = 5;

    // Traditional if
    if a > 5 {
        println!("a is greater than 5");
    } else if a > 3 {
        println!("a is greater than 3");
    } else {
        println!("smaller or equal to 3");
    }

    //  Expression if. Boths branch must be present and return the same value type.
    let b = if a > 5 { 1 } else { -1 };

    // loop (infinity loop)
    'outer: loop {
        println!("loop forever");
        loop {
            break 'outer;
        }
    }

    // loop can return a value.
    let x = loop {
        break 5;
    };

    let mut a = 0;

    while a < 5 {
        println!("Valor de a: {a}");
        a += 1;
    }

    let a = [1, 2, 3, 4, 5];

    // The expression after "in" must implement IntoIter trait.
    // for loop uses the into_iter method of the value to obtain a iterator.
    for element in a {
        println!("Valor de element: {element}");
    }
}
