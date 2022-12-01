#![allow(unused_variables)]

// Find the user-defined geo mod
mod geo;

// Pull in the distance function part of the Calculation mod
// added an alias, this is used when there are multiple distance
// Rust components so they can be uniquely called
use geo::calculations::distance as distance_calc;

// Setup in dependencies section of Cargo.toml file
use rand::Rng;

fn main() {
    let kcle_latitude_degrees: f64 = 41.4075;
    let kcle_longitude_degrees: f64 = -81.851111;

    let kslc_latitude_degrees: f64 = 40.7861;
    let kslc_longitude_degrees: f64 = -111.9822;

    // Standard data types and associated methods
    // https://doc.rust-lang.org/std/index.html
    // Execute the distance function from the geo::caluclations mod
    //let distance = geo::calculation::distance(kcle_latitude_degrees, kslc_latitude_degrees, kcle_longitude_degrees, kslc_longitude_degrees);
    //let distance = distance(kcle_latitude_degrees, kslc_latitude_degrees, kcle_longitude_degrees, kslc_longitude_degrees);

    let distance = distance_calc(kcle_latitude_degrees, kslc_latitude_degrees, kcle_longitude_degrees, kslc_longitude_degrees);
    println!("The distance between the two points is {:.1} kilometers", distance);

    generate_random();
}

fn generate_random() {
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();
    println!("{}", random_number);
}