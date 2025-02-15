use std::fmt::Debug;

trait Move {
    fn move_2d(&self);
    fn move_3d(&self);
}

pub enum Toy {
    Car(ToyCar),
    Plane(ToyPlane),
    Boat(ToyBoat),
}

#[derive(Debug)]
pub struct ToyCar {
    pub color: String,
    pub wheels: u8,
}

#[derive(Debug)]
pub struct ToyPlane {
    pub color: String,
    pub wings: u8,
    pub capacity: f32,
    pub model: String,
}

pub struct ToyBoat {
    pub color: String,
    pub length: u8,
    pub capacity: f32,
}
//implementation using match and enum
impl Move for Toy {
    fn move_2d(&self) {
        match self {
            Toy::Car(_) => println!("The car rolls forward in 2D."),
            Toy::Boat(_) => println!("The boat moves forward in 2D."),
            Toy::Plane(_) => println!("This plane cannot move in 2D."),
        }
    }

    fn move_3d(&self) {
        match self {
            Toy::Car(_) => println!("This car cannot move in 3D."),
            Toy::Boat(_) => println!("This boat cannot move in 3D."),
            Toy::Plane(_) => println!("The plane flies forward in 3D."),
        }
    }
}

pub fn egs() {
    let my_car = Toy::Car(ToyCar {
        color: String::from("blue"),
        wheels: 4,
    });

    let my_plane = Toy::Plane(ToyPlane {
        color: String::from("blue"),
        wings: 2,
        capacity: 800.0,
        model: String::from("2014"),
    });

    let my_boat = Toy::Boat(ToyBoat {
        color: String::from("white"),
        length: 15,
        capacity: 20.2,
    });

    // Use the enum with trait methods
    my_car.move_2d();
    my_car.move_3d();

    my_boat.move_2d();
    my_boat.move_3d();

    my_plane.move_2d();
    my_plane.move_3d();

    // Display toy details
    if let Toy::Car(car) = &my_car {
        println!("My car: color: {:?}, wheels: {:?}", car.color, car.wheels);
    }
    if let Toy::Boat(boat) = &my_boat {
        println!(
            "My boat: color: {:?}, length: {:?}, capacity: {:?}",
            boat.color, boat.length, boat.capacity
        );
    }
    if let Toy::Plane(plane) = &my_plane {
        println!(
            "My plane: capacity: {:?}, color: {:?}, model: {:?}, wings: {:?}",
            plane.capacity, plane.color, plane.model, plane.wings
        );
    }
}