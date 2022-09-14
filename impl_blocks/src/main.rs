#[allow(dead_code)]
#[derive(Debug)]
struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

impl Product {
    // 3 possibles "sel"
    // &self
    // &mut self
    // self
    fn calculate_sales_tax(&self) -> f32 {
        self.price * 0.1
    }

    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    fn buy(self) -> i32 {
        let name = self.name;
        println!("{name} was bought!");
        123
    }

    // Associated function of Product type.
    // Its like static method.
    // Does not need instance of Type.
    fn get_default_sales_tax() -> f32 {
        0.1
    }

    // "new" os a common pattern inf Rust as a assoicated function to create a new value of the type.
    // Its is like a constructor.
    fn new(name: String, price: f32) -> Self {
        // The real constructor is the type name, but the "new" associated function gives more control.
        Product {
            name: name,
            price: price,
            in_stock: true,
        }
    }
}

#[allow(unused_variables)]
fn main() {
    let mut book = Product {
        name: String::from("Book"),
        price: 4.99,
        in_stock: true,
    };

    let price = book.price;
    book.in_stock = false;

    let sales_tax = book.calculate_sales_tax();
    println!("{sales_tax}");

    book.set_price(1.0);
    book.buy();
    println!("{}", Product::get_default_sales_tax());

    #[allow(unused_mut)]
    let mut book = Product::new(String::from("Book"), 30.0);
    println!("{:?}", book);
}
