struct Tweet<'a> {
    content: &'a str,
}

// Elide rules
// 1. Each parameter thas is a reference gets its own lifetime parameter.
// 2. If there is exactly one inputed lifetime parameter, that lifetime parameter
//    is assigned to all outputs lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is
//    &self or &mut self, the lifetime of self is assigned to all outputs lifetimes parameters.
impl<'a> Tweet<'a> {
    // The third rule applies.
    fn replace_content(&mut self, content: &'a str) -> &str {
        let old_content = self.content;
        self.content = content;
        old_content
    }
}

// The first 2 rules appplies. No annotations are needed.
#[allow(dead_code)]
fn take_and_return_content(content: &str) -> &str {
    content
}

// Equivalente with lifetime annotations
#[allow(dead_code)]
fn take_and_return_content2<'a>(content: &'a str) -> &'a str {
    content
}

// The three rules dos not apply, you need to annotate the lifetimes.
// In this example, output/result is related to content, as both has the same lifetime and its
// different of _content2
#[allow(dead_code)]
fn take_and_return_content3<'a, 'b>(content: &'a str, _content2: &'b str) -> &'a str {
    content
}

// The three rules dos not apply, you need to annotate the lifetimes.
// In this example, output/result is related to boths parameters.
// The compiler assumes the shortest lifetime of content and content2.
#[allow(dead_code)]
fn take_and_return_content4<'a>(content: &'a str, content2: &'a str) -> &'a str {
    if content.len() <= content2.len() {
        content
    } else {
        content2
    }
}
fn main() {
    // Its is better to watch the videos.
    // The compiler checks that, when a value is used, it is still valid, it was not dropped.
    // It also checks references. A reference lifetime must be contained in the lifetime of the referenced value.
    // The compiler is able to detect "no lexical lifetime".
    // If a reference is not used after a point in the program, the compiler assumes its lifetime ended.
    println!("Hello, world!");
    let mut tweet = Tweet { content: "example" };
    let old_content = tweet.replace_content("replace_example");
    println!("{old_content}");
    println!("{}", tweet.content);
}
