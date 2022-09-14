#[allow(dead_code)]
struct Product {
    name: String,
    price: f32,
    in_stock: bool,
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

    let sales_tax = calculate_sales_tax(&book);
    println!("{sales_tax}");
}

fn calculate_sales_tax(product: &Product) -> f32 {
    product.price * 0.1
}
