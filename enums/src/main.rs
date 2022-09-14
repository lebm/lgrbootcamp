#[allow(dead_code)]
enum ProductCategory {
    Books,
    Clothing,
    Electrics,
}

#[allow(dead_code)]
struct Product {
    name: String,
    category: ProductCategory,
    price: f32,
    in_stock: bool,
}

#[allow(dead_code)]
enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace {
        from: String,
        to: String,
    },
}

// You can create impl block to enums
impl Command {
    fn serialize(&self) -> String {
        String::from("JSON string")
    }
}

#[allow(unused_variables)]
fn main() {
    let cmd = Command::Undo;
    let cmd = Command::AddText(String::from("test"));
    let cmd = Command::MoveCursor(22, 0);
    let cmd = Command::Replace {
        from: String::from("a"),
        to: String::from("b"),
    };

    let json_string = cmd.serialize();
    println!("{json_string}");
}
