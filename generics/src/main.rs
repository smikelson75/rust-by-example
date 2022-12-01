use std::ops::{Add, Sub};
// #[derive(Debug)]
// struct NdbNavAid {
//     name: String,
//     frequency: u16
// }

// #[derive(Debug)]
// struct VorNavAid {
//     name: String,
//     frequency: f32
// }

#[derive(Debug)]
struct NavAid<T, U> {
    name: String,
    frequency: T,
    data: U
}


fn main() {
    let vor = NavAid {
        name: String::from("DQN"),
        frequency: 114.5,
        data: String::from("DQN is a VOR")
    };

    let ndb_data: Option<String> = Option::None;
    let ndb = NavAid {
        name: String::from("HKF"),
        frequency: 239,
        data: ndb_data
    };

    println!("VOR information is: {:?}", vor);
    println!("NDB information is: {:?}", ndb);

    let sum = add(256, 262);
    println!("{}", sum);

}

// Define Basic constraint
//fn add<T: Add<Output = T>>(operand1: T, operand2: T) -> T {

// Define multiple contraints with where clause
fn add<T>(operand1: T, operand2: T) -> T
where T: Add<Output = T> + Sub<Output = T> {
    operand1 + operand2
}