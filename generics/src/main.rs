// payload may have values type we don't know yet, so let's make it generic.
// "T" is the type paramter used for payload field.
// This struct may be BrowserCommand<String>, BrowserCommand<i32> ena so on,
#[allow(unused_variables)]
struct BrowserCommand<T> {
    name: String,
    payload: T,
}

impl<T> BrowserCommand<T> {
    // Self is the type "owning" this impl block: Self = BrowserCommand<T>
    fn new(name: String, payload: T) -> Self {
        // Using a variable with the same name of the field is a ergonomic shorthand.
        // You don't have to write "name: name", or "payload: payload"
        BrowserCommand { name, payload }
    }
}

// This impl block specifies to String. It adds to BrowserCommand<String>
impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }
}

#[allow(unused_variables)]
fn main() {
    // cmd1 is BrowserCommand<String>
    let cmd1 = BrowserCommand::new(
        "navigate".to_owned(),
        "https://www.letsgetrusty.com".to_owned(),
    );

    // cmd2 is BrowserCommand<i32
    let cmd2 = BrowserCommand::new("zoom".to_owned(), 200);
    cmd1.print_payload();
    // BrowserCommand<i32> does not have a print_payload method
    //cmd2.print_payload();
}
