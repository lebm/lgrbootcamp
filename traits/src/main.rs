trait Park {
    /// In this example, the method oerk dos not has a body. Each type implementing the trait will implement specifi park method.  
    fn park(&self);
}

trait Paint {
    /// This trait method has a body, in this case the method  is a default implementation, bu it can be overridden.
    fn paint(&self, color: &str) {
        println!("Painting object: {color}");
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}
/// This  is not inheritance!!  
#[allow(dead_code)]
#[derive(Debug)]
struct Car {
    info: VehicleInfo,
}

/// "Impl TRAIT for TYPE" defines a block that implements the necessary methods for a type so that it can "belong" to the trait.
/// In a sensa, a trait is a kind of class, or category, bu its nto a class ina a OO sense.
/// NOTE: Only functionality can be shared, unlike OO, data os not shared (there is no inheritance).
impl Park for Car {
    fn park(&self) {
        println!("parking car!");
    }
}

/// Paint has only default methods, so the impl block  can be empty, the type  will use the default methods.
impl Paint for Car {}

#[allow(dead_code)]
#[derive(Debug)]
struct Truck {
    info: VehicleInfo,
}

impl Park for Truck {
    fn park(&self) {
        println!("parking truck!");
    }
}

impl Paint for Truck {}

impl Truck {
    fn unload(&self) {
        println!("Unloading truck.");
    }
}

struct House {}

/// House is completely different from a car or truck, nut we can paint it.   
/// Completely different kind of values con implement the same trait.  
/// In this example, we override paint method for the House type.
impl Paint for House {
    fn paint(&self, color: &str) {
        println!("Painting house: {color}");
    }
}

/// Wrong. In this example "T" can be any type, but the compiler can find a paint method for any type.
/// "paint" exists only for values that implements the Paint trait.
// fn paint_red<T>(object: &T) {
//     object.paint("red");
// }

/// Ok. In this example "T" can be any type as long as it implements the Paint trait. (trait boud)
fn paint_red<T: Paint>(object: &T) {
    object.paint("red");
}

/// Another example ...
fn paint_blue(object: &impl Paint) {
    object.paint("blue");
}

/// Third way
fn paint_green<T>(object: &T)
where
    T: Paint,
{
    object.paint("green");
}

/// two bounds
fn paint_vehicle_green<T>(object: &T)
where
    T: Paint + Park,
{
    object.paint("green");
}

/// You can return a trait bound
fn create_paintable_object() -> impl Paint {
    House {}
}

fn main() {
    println!("Hello, world!");
    let carro = Car {
        info: VehicleInfo {
            make: "vw".to_owned(),
            model: "fusca".to_owned(),
            year: 1967,
        },
    };
    let caminhao = Truck {
        info: VehicleInfo {
            make: "mercedes".to_owned(),
            model: "1980".to_owned(),
            year: 1980,
        },
    };
    let casa = House {};
    println!("{carro:?} {caminhao:?}");
    carro.paint("azul");
    caminhao.paint("vermelho");
    casa.paint("branco");
    carro.park();
    caminhao.park();
    caminhao.unload();
    paint_red(&carro);
    paint_blue(&caminhao);
    paint_green(&casa);
    paint_vehicle_green(&caminhao);
    paint_vehicle_green(&carro);
    let casa2 = create_paintable_object();
    paint_green(&casa2);
}
