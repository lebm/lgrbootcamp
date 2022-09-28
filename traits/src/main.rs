/// In the example, values that implement Vehicle musta also implement Paint.
/// Paint is more generic then Vehicle.
/// Every vehicle can be painted, but there are values that can be painted that is not vehicle.
/// Paint is a supertrait of Vehicle.
trait Vehicle: Paint {
    /// In this example, the method park dos not has a body.
    /// Each type implementing the trait will implement specific park method.  
    fn park(&self);
    /// get_default_color is an associated function of Vehicle trait.
    /// It is associated with the trait, not a value that implements it.
    /// There is no need to pass self as the first parameter.
    /// It is invoked with the "::" operator.
    fn get_default_color() -> String {
        "black".to_owned()
    }
}

trait Paint {
    /// This trait method has a body, in this case the method is a default implementation, but it can be overridden.
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
impl Vehicle for Car {
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

impl Vehicle for Truck {
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

/// House is completely different from a car or truck, but we can paint it.   
/// Completely different kind of values con implement the same trait.  
/// In this example, we override paint method for the House type.
impl Paint for House {
    fn paint(&self, color: &str) {
        println!("Painting house: {color}");
    }
}

/// Wrong. In this example "T" can be any type, but the compiler can not find a paint method for any type.
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
fn paint_green(object: &dyn Paint) {
    object.paint("green");
}

/// two bounds
/// There is no need to mention Paint in the bound, as Paint is a supertrait of Vehicle.
fn paint_vehicle_green<T>(object: &T)
where
    T: Vehicle,
{
    object.paint("green");
}

/// A function can return a trait bound (impl TRAIT, but only if it is returning a concrete type.
/// If the function returns diferent types, it must return a trait object.
fn create_paintable_object(vehicle: bool) -> Box<dyn Paint> {
    if vehicle {
        Box::new(Car {
            info: VehicleInfo {
                make: "vw".to_owned(),
                model: "fusca".to_owned(),
                year: 1967,
            },
        })
    } else {
        Box::new(House {})
    }
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
    println!("Default color of Car: {}", Car::get_default_color());
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
    let casa2 = create_paintable_object(true);
    paint_green(casa2.as_ref());

    // carro and casa are of different types, but both implements Paint.
    // The example below creates a vector of traits objects of different types but that implement a common trait.
    let paintable_objects: Vec<&dyn Paint> = vec![&carro, &casa];
    for o in &paintable_objects {
        o.paint("red");
    }
}
