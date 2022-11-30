#![allow(unused)]

struct Boeing {
    required_crew: u8,
    range: u16
}

struct Airbus {
    required_crew: u8,
    range: u16
}

trait Flight {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool;
}

impl Flight for Boeing {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
        if (available_crew >= required_crew) && (range + 150 > distance) {
            true
        } else {
            false
        }
    }
}

impl Flight for Airbus {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
        if (available_crew >= required_crew) && (range + 280 > distance) {
            true
        } else {
            false
        }
    }
}


fn main() {
    let boeing = Boeing {
        required_crew: 4,
        range: 7370
    };

    let airbus = Airbus {
        required_crew: 7,
        range: 5280
    };

    let is_boeing_legal = boeing.is_legal(boeing.required_crew, 18, boeing.range, 2385);
    let is_airbus_legal = airbus.is_legal(airbus.required_crew, 3, airbus.range, 2200);

    println!("Is the Boeing flight legal? {}\nIs the Airbus flight legal? {}", is_boeing_legal, is_airbus_legal);
}
