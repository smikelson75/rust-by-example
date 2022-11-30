#![allow(unused)]

fn main() {
    // Call a function and return a value
    let greater = return_greater(91, 22);
    // output: 91
    println!("{}", greater);

    let mut original = String::from("original value");
    println!("\nouter scope original:\t\"{}\"", original);

    {
        // Call function by reference, returns nothing
        print_original(&original);

        // Call function by reference, allows value to be changed
        change_original(&mut original);
        println!("inner scope original: \t\"{}\"", original);
    }

    let name = "Duck Airlines";

    // Closure (Anonymous function) with no parameters
    let write_message = || {
        println!("Hey, {}. This is the closure", name);
    };

    write_message();

    // Closure (Anonymous function) with parameters and return value
    let write_message_param = |slogan: String| -> String {
        String::from(format!("{}. {}", name, slogan))
    };

    let phrase = write_message_param(String::from("We hit the ground every time."));
    print!("{}", phrase);
}

// Function:
// first:
//    unsigned 8 bit
//    pass by value
// second:
//    unsigned 8 bit
//    pass by value
// returns:
//    unsigned 8 bit
fn return_greater(first: u8, second: u8) -> u8 {
    if first > second {
        first
    } else {
        second
    }
}

// Function:
// original:
//    string
//    pass by reference
// returns:
//    nothing
fn print_original(original: &String) {
    println!("fn print_original: \t\"{}\"", original);
}

// Function:
// original:
//    string (mutable)
//    pass by reference
// returns:
//    nothing
fn change_original(original: &mut String) {
    let next = original;
    *next = String::from("next value");
}
