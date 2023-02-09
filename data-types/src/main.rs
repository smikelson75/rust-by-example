#![allow(unused_variables)]

fn main() {
    // Scalar data types 
    // integer, float, boolean, char
    
    // Integer 
    // Unsigned Integer, u8, u16, u32, u64
    let unsigned_integer: u8 = 255;

    // Signed Integer: i8, i16, i32, i64
    let signed_integer: i8 = -128;

    // Floats: f32, f64
    let float_32_bytes: f32 = 100.123;

    // Boolean: bool (true, false)
    let true_false: bool = true;

    // Character: char
    let character_8_bytes: char = 'A';

    println!("Scalar values defined {:?}", (unsigned_integer, signed_integer, float_32_bytes, true_false, character_8_bytes));

    // Compound data types
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
    println!("{}", slogan);

    // Array (https://doc.rust-lang.org/rust-by-example/primitives/array.html)
    let location_array: [f32;2] = [41.4094069, -81.8546911];
    let abbreviation: &str = "KCLE";
    println!("Location Name: {}, Latitude: {}, Longitude: {}", abbreviation, location_array[0], location_array[1]);

    // destructuring array
    let [latitude1, longitude1] = location_array;
    println!("Location Name: {}, Latitude: {}, Longitude: {}", abbreviation, latitude1, longitude1);

    // tuple (https://doc.rust-lang.org/rust-by-example/primitives/tuples.html)
    let location_tuple: (&str, f64, f64) = ("KCLE", 41.4094069, -81.8546911);
    println!("Location Name: {}, Latitude: {}, Longitude: {}", location_tuple.0, location_tuple.1, location_tuple.2);

    // destructuring tuple
    let (name, latitude, longitude) = location_tuple;
    println!("Location Name: {}, Latitude: {}, Longitude: {}", name, latitude, longitude);
}