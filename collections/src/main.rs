use std::collections::{VecDeque, HashMap, HashSet};

fn main() {
    sequences_vector();
    sequences_vecdeque();
    maps();
    sets();
}

fn sequences_vector() {
    // Define a vector
    let mut flights: Vec<&str> = Vec::new();

    // Use macro to define a vector
    //let vec_macro = vec![1, 2, 3, 4];

    // Add values to the vector
    flights.push("DA113\tto Boston departs at 06:20");
    flights.push("DA98\tto London departs at 09:43");
    flights.push("DA428\tto Salt Lake City departs at 12:05");
    flights.push("DA41\tto Berlin departs at 15:30");
    flights.push("DA2815\tto Nashville departs at 17:11");

    // Iterate through the vector
    for flight in flights.iter() {
        println!("{}", flight);
    }

    // Fast, less safe way to retrieve a value
    // If pointing to an index that doesn't exist,
    // program will panic
    let third = flights[2];
    println!("{}", third);

    // Safer way to get a value from the vector, results in
    // Option to run a match on
    let fourth = flights.get(3);

    // What to do if no data is returned.
    match fourth {
        Some(value) => {
            println!("{}", value);
        }
        _ => {
            println!("No value provided.")
        }
    }

    // Only do something is a value is returned.
    if let Some(flight_value) = flights.get(4) {
        println!("{}", flight_value);
    }

    flights.insert(2, "DA918\tto Orlando departs at 11:12");

    // Iterate through the vector
    for flight in flights.iter() {
        println!("{}", flight);
    }

    flights.remove(1);

    // Iterate through the vector
    for flight in flights.iter() {
        println!("{}", flight);
    }
}

fn sequences_vecdeque() {
    // Vector Double Ended Queue
    // Can add to both front and back
    // Can't sort the elements
    let mut flights:VecDeque<&str> = VecDeque::new();

    flights.push_front("DA918\tto Orlando departs at 11:12");
    flights.push_back("DA428\tto Salt Lake City departs at 12:05");
    flights.push_front("DA98\tto London departs at 09:43");
    flights.push_front("DA113\tto Boston departs at 06:20");
    flights.push_back("DA2815\tto Nashville departs at 17:11");

    // Iterate through the vector
    for flight in flights.iter() {
        println!("{}", flight);
    }

    let length = flights.len();
    println!("{}", length);

    // Looks for the exact match
    let exists = flights.contains(&"DA98\tto London departs at 09:43");
    println!("Element exists: {}", exists);

    flights.clear();

    let length = flights.len();
    println!("{}", length);
}

fn maps() {
    // HashMap
    let mut flights = HashMap::new();

    // Insert values into HashMap
    flights.insert("DA918", ("11:12", "Orlando"));
    flights.insert("DA428", ("12:05", "Salt Lake City"));
    flights.insert("DA98",  ("09:43", "London"));
    flights.insert("DA113", ("06:20", "Boston"));
    flights.insert("DA41",  ("15:30", "Berlin"));
    flights.insert("DA2815", ("17:11", "Nashville"));

    let flight_number = "DA2815";

    // Find value (tuple of time and place)
    let option = flights.get(flight_number);

    // Destructure value if Some, panic if nothing
    let (time, destination) = option.unwrap();

    // Write to console
    println!("{} {}", time, destination);

    // There is no automatic collision detection.
    // Test for existing or if it exists, it will be overwritten
    if !flights.contains_key(flight_number) {
        flights.insert(flight_number, ("12:00", "Puerto Rico"));
    } else {
        println!("Flight {} is already entered.", flight_number);
    }

    let flight_number = "DA531";

    // There is no automatic collision detection.
    // Test for existing or if it exists, it will be overwritten
    if !flights.contains_key(flight_number) {
        flights.insert(flight_number, ("12:00", "Puerto Rico"));
    } else {
        println!("Flight {} is already entered.", flight_number);
    }

    // Iterate through the vector
    for flight in flights.iter() {
        println!("{:?}", flight);
    }
}

fn sets() {
    let mut flights = HashSet::new();

    flights.insert(("DA918", "11:12", "Orlando"));
    flights.insert(("DA428", "12:05", "Salt Lake City"));
    flights.insert(("DA98",  "09:43", "London"));
    flights.insert(("DA113", "06:20", "Boston"));
    flights.insert(("DA41",  "15:30", "Berlin"));
    flights.insert(("DA2815", "17:11", "Nashville"));

    // Iterate through the HashSet
    // Random order is written, no particular order
    for flight in flights.iter() {
        println!("{:?}", flight);
    }
}