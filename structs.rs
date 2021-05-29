#[derive(Clone)]

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }
}

fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 0.0
    };

    let vehicle2 = Shuttle {
        //name: String::from("Discovery"),
        ..vehicle.clone()
    };

    println!("name is {}", vehicle.name);

    vehicle.name = String::from("Atlantis");
    println!("vehicle is {:?}", vehicle);
    println!("vehicle2 is {:?}", vehicle2);
}