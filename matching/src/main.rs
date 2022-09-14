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
        let json_string = match self {
            Command::Undo => String::from(r#"{"cmd": "undo"}"#),
            Command::Redo => String::from(r#"{"cmd": "redo"}"#),
            Command::AddText(s) => {
                format!(r#"{{"cmd": "addText", "text": "{s}"}}"#)
            }
            Command::MoveCursor(line, column) => {
                format!(r#"{{"cmd": "move_cursor", "line": "{line}, "column": {column}}}"#)
            }
            Command::Replace { from, to } => {
                format!(r#"{{"cmd": "replace", "from": "{from}", "to": "{to}"}}"#)
            }
        };
        json_string
    }
}

fn main() {
    let cmd1 = Command::Undo;
    let cmd2 = Command::AddText(String::from("test"));
    let cmd3 = Command::MoveCursor(22, 0);
    let cmd4 = Command::Replace {
        from: String::from("a"),
        to: String::from("b"),
    };
    let cmd5 = Command::Redo;

    println!("{}", cmd1.serialize());
    println!("{}", cmd2.serialize());
    println!("{}", cmd3.serialize());
    println!("{}", cmd4.serialize());
    println!("{}", cmd5.serialize());

    // let age = 35;

    // // match is exhaustive. arms must match all possible values.
    // // "-" matches the rest, like a default arm.
    // match age {
    //     1 => print!("Happy 1st Birthday!"),
    //     13..=19 => println!("You are teenager!"),
    //     _ => println!("Happy Birthday"),
    // }

    // // 2nd example
    // // The x var captures whatever values age
    // match age {
    //     1 => print!("Happy 1st Birthday!"),
    //     13..=19 => println!("You are teenager!"),
    //     x => println!("You are {x} years old!"),
    // }
}
