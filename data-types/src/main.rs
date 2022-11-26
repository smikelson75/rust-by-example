#![allow(unused_variables)]

fn main() {
    // Unsigned Integer
    let unsigned_integer: u8 = 255;
    // Signed Integer
    let signed_integer: i8 = -128;

    println!("Unsigned Max: {}, Signed Minimum: {}", unsigned_integer, signed_integer);

    // array
    let location_array: [f32;2] = [41.4094069, -81.8546911];
    let abbreviation: &str = "KCLE";
    println!("Location Name: {}, Latitude: {}, Longitude: {}", abbreviation, location_array[0], location_array[1]);

    // tuple
    let location_tuple: (&str, f64, f64) = ("KCLE", 41.4094069, -81.8546911);
    println!("Location Name: {}, Latitude: {}, Longitude: {}", location_tuple.0, location_tuple.1, location_tuple.2);

    // destructuring tuple
    let (name, latitude, longitude) = location_tuple;
    println!("Location Name: {}, Latitude: {}, Longitude: {}", name, latitude, longitude);

    // destructuring array
    let [latitude1, longitude1] = location_array;
    println!("Location Name: {}, Latitude: {}, Longitude: {}", abbreviation, latitude1, longitude1);

    // Strings
    let person_name_slice: &str = "Donald Mallard"; // string slice
    let person_name_string = person_name_slice.to_string(); // String from string slice
    let person_name_string_from = String::from(person_name_slice); // String from string slice version 2
    let person_to_string_slice = person_name_string.as_str(); // String to string slice

    // String concatenation
    let duck = "Duck"; // string slice
    let airlines = "Airlines";

    let airline_name = [duck, " ", airlines].concat(); // String
    println!("{}", airline_name);

    let airline_name_format = format!("{} {}", duck, airlines);
    println!("{}", airline_name_format);

    let mut slogan = String::new();
    slogan.push_str("We hit the ground");
    slogan.push(' ');
    slogan = slogan + "every time";
    println!("{}", slogan)
}