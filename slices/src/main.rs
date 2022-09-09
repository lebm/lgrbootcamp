#[allow(unused_variables)]
fn main() {
    // WARNING: It works because this string is just ASCII, but with a real UTF-8 string, taking an arbitrary slice
    // may cause panic.
    let s1 = String::from("The quick brow fox jumps over the lazy dog.");
    let rs1 = &s1[..20];
    println!("{rs1}");

    /*
       String: heap allocated and growable UTF-8 sequence. Can be mutable and relocated.
       str: fixed UTF-8 anywhere in memory. Can not be used as is because the compiler does not know is size.
          so it almost always used as &str. When someone talks about "string slice", he/she talking about "&str"

       All string literals are strings slices and are saved with the program binary.
    */

    let s2 = String::from("The quick brow fox jumps over the lazy dog.");
    let rs2 = trim_string1(&s2);
    println!("{rs2}");

    let s3 = "The quick brow fox jumps over the lazy dog.";
    let rs3 = trim_string2(s3);
}

// Problem with this function. It only accepts &String
fn trim_string1(s: &String) -> &str {
    &s[..20]
}

/*
This version accepts &String both &String and &str
&String os converted to &str by the compiler (deref coercion)
Most &String methods actually are &str methods. When a &str method os applied to &String, deref coercion occurs, so that the method can be applied.
*/
fn trim_string2(s: &str) -> &str {
    &s[..20]
}
