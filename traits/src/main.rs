#![allow(unused)]

struct Boeing {
    required_crew: u8,
    range: u16
}

struct Airbus {
    required_crew: u8,
    range: u16
}

// Trait provides a function signature to be implemented by
// impl for structs, similar to an interface in other languages.
trait Flight {
    fn is_legal(&self, available_crew: u8, distance: u16) -> bool;
}

// Trait implementation for the Boeing struct
impl Flight for Boeing {
    fn is_legal(&self, available_crew: u8, distance: u16) -> bool {
        if (available_crew >= self.required_crew) && (self.range + 150 > distance) {
            true
        } else {
            false
        }
    }
}

// Trait implementation for the Airbus struct
impl Flight for Airbus {
    fn is_legal(&self, available_crew: u8, distance: u16) -> bool {
        if (available_crew >= self.required_crew) && (self.range + 280 > distance) {
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

    let is_boeing_legal = boeing.is_legal(18, 2385);
    let is_airbus_legal = airbus.is_legal(3, 2200);

    println!("Is the Boeing flight legal? {}\nIs the Airbus flight legal? {}", is_boeing_legal, is_airbus_legal);
}
