// A trait with one default method.
trait UIComponent {
    fn render(&self) {
        println!("Rendering component...");
    }
}

#[allow(dead_code)]
struct Button {
    text: String,
}

// The block can be empty. UIComponente only has default methods
impl UIComponent for Button {}

// Recursive types has infinite size. The compiler connot allocate this value.
// A indirection like Box, RC or "&" solves this problem
#[allow(dead_code)]
struct Container {
    name: String,
    child: Box<Container>,
}

impl UIComponent for Container {}

fn main() {
    // allocated on the stack
    let _button_a = Button {
        text: "button a".to_owned(),
    };

    // allocated on the heap
    // Owned referece
    let _button_b = Box::new(Button {
        text: "button b".to_owned(),
    });

    // Ownership moved.
    // The role _button_a is moved to _button_c on the stack.
    let _button_c = _button_a;

    // The _button_b metadata (reference) on the stack is moved to _button_d,
    // but the content on the heap is not touched.
    let _button_d = _button_b;

    // Box is also used with trait objects
    // components is a vector of values that implement UIComponente trait.
    // Values of different types can implement UIComponente, the compiler will not know at compile time the size
    // of each array element, so we must use trait objects inside a Box.
    let _components: Vec<Box<dyn UIComponent>> = vec![Box::new(_button_c), _button_d];
}
