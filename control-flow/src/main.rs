#![allow(unused_variables)]

fn main() {
    // If/Else (https://doc.rust-lang.org/rust-by-example/flow_control/if_else.html)
    let word = "Duck";

    // If then
    if word == "Duck" {
        println!("Quack");
    }

    // If let (https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
    // if let word = "Cat" {
    //     println!("Meow");
    // }

    // If then ... else
    if word == "Duck" {
        println!("Quack");
    } else if word == "Dog" {
        println!("Bark");
    } else {
        println!("All quiet out here");
    }

    let available_aircraft = "Boeing";
    let minimum_crew = 7;
    let available_crew = 4;

    // if then using logical and/or with 
    // order of operations
    if (available_aircraft == "Boeing" ||
        available_aircraft == "Airbus") &&
        minimum_crew <= available_crew {
            print!("Okay");
    }

    // Enumeration (https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)
    enum NaviagationAidsBasic {
        NDB,
        VOR,
        VORDME
    }

    // Rust doesn't assume an enumeriation is numeric, however,
    // by default is integer 0, 1, 2, etc.
    println!("NDB:\t{}", NaviagationAidsBasic::NDB as u8);
    println!("VOR:\t{}", NaviagationAidsBasic::VOR as u8);
    println!("VORDME:\t{}", NaviagationAidsBasic::VORDME as u8);

    // Rust does allow you to set values explicitly
    // and the value is added to by 1
    enum NaviagationAidsSetVOR {
        NDB,        // 0
        VOR = 3,    // 3
        VORDME      // 4
    }

    println!("NDB:\t{}", NaviagationAidsSetVOR::NDB as u8);
    println!("VOR:\t{}", NaviagationAidsSetVOR::VOR as u8);
    println!("VORDME:\t{}", NaviagationAidsSetVOR::VORDME as u8);

    // Rust does allow you to set values explicitly
    // If VORDME is left with no value, the code will NOT compile
    // because the value will be the same ad NDB
    enum NaviagationAidsSetValues {
        NDB = 3,
        VOR = 2,
        VORDME = 5
    }

    println!("NDB:\t{}", NaviagationAidsSetValues::NDB as u8);
    println!("VOR:\t{}", NaviagationAidsSetValues::VOR as u8);
    println!("VORDME:\t{}", NaviagationAidsSetValues::VORDME as u8);

    // No Null data type
    let phrase = String::from("Duck Airlines");
    let letter = phrase.chars().nth(5); // Create an array of characters, and get the value in the 5th position using 0-based arrays, "A"

    // Instead of the value being return (or a null) the Option Enumeriation is returned
    // It contains two values, Some or None
    let value = match letter {
        // If Some is returned, get the stored the character as a string and store into value
        Some(character) => character.to_string(),
        // If None is returned, no value was returned (outside the limit of array)
        // now, store the string "No value" in value
        None => String::from("No value")
    };

    println!("{}", value);

    // Match statement (https://doc.rust-lang.org/rust-by-example/flow_control/match.html)
    // similar to switch statement
    let animal = "Duck";
    match animal {
        // animal contains duck, print "Quack"
        "Duck" => println!("Quack"),
        // animal contains Dog, print "Bark"
        "Dog" => println!("Bark"),
        // for all other values, print "All quiet out here"
        // All values must be accounted for, else is required
        _ => println!("All quiet out here")
    }

    let ndb_freq:u16 = 384;

    // Match using range of values
    match ndb_freq {
        // ndb_freq is between 200 and 500 inclusive
        // Cannot write exclusive range using this format
        200..=500 => {
            println!("NDB Frequency is valid");
        }
        _ => {
            println!("NDB Frequency is not valid");
        }
    }

    // Match using if syntax for more complex testing
    match ndb_freq {
        // ndb_freq is between 200 and 500 inclusive
        ndb_freq if ndb_freq >= 200 && ndb_freq <= 500 => {
            println!("NDB frequency is valid");
        }
        _ => {
            println!("NDB Frequency is not valid");
        }
    }

    // Enumerations, Part II
    // See definition of enum below
    let ndb_uwl = NaviagationAids::NDB(385);
    let vor_dqn = NaviagationAids::VOR(String::from("DQN"), 114.5);
    let vor_dme_sgn = NaviagationAids::VORME(String::from("SGH"), 113.2);
    let fix_tarry = NaviagationAids::FIX { 
        name: String::from("TARRY"), 
        latitude: 40.05333, 
        longitude: -83.91367 
    };

    print_nav_aid(&ndb_uwl);
    print_nav_aid(&vor_dqn);
    print_nav_aid(&vor_dme_sgn);
    print_nav_aid(&fix_tarry);

}

// Enumeration, Part II
enum NaviagationAids {
    NDB(u16),
    VOR(String, f32),
    VORME(String, f32),
    FIX{name: String, latitude: f32, longitude: f32}
}

fn print_nav_aid(navaid: &NaviagationAids) {
    match navaid {
        NaviagationAids::NDB(khz) => {
            println!("NDB frequency is {} kilohertz", khz);
        }
        NaviagationAids::VOR(id, freq) => {
            println!("VOR identifier is {} and it's frequency is {} kilohertz", id, freq);
        }
        NaviagationAids::VORME(id, freq) => {
            println!("VORDME identifier is {} and it's frequency is {} kilohertz", id, freq);
        }
        NaviagationAids::FIX { name, latitude, longitude } => {
            println!("FIX {} is at {} latitude and {} longitude", name, latitude, longitude);
        }
    }
}
