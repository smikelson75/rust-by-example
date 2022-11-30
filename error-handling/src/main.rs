use std::fs::File;
use std::io::{ErrorKind, Error, Read};

fn main() {
    // Panic error, unrecoverable issue in code
    // use RUST_BACKTRACE=1 to see stack trace
    // panic!("Crash & Burn!")

    // File path
    let filename = "C:\\Temp\\customer.json";

    // Attempt to open file, File::open() returns enum
    // for matching
    match File::open(filename) {
        // File opened successfully, read
        Ok(file) => {
            println!("{:#?}", file);
        }

        // File failed to open
        Err(error) => {
            // Check the ErrorKind
            match error.kind() {
                // If ErrorKind is NotFound
                ErrorKind::NotFound => {
                    // Try to create file, return enum 
                    match File::create(filename) {
                        // File was created successfully
                        Ok(_file) => {
                            println!("File created.")
                        }
                        // File failed to create.
                        Err(error) => {
                            println!("{:#?}", error);

                        }
                    }
                }
                // Some other ErrorKind was encountered
                _ => {
                    // Print the error to the console.
                    println!("{:#?}", error);
                }
            }
        }
    }

    let file_data  = read_file(filename);

    match file_data {
        Ok(data) => {
            println!("{}", data);
        }
        Err(_) => {}
    }

}

// Function allows for the Error to be passed back up the stack
// Propagating error
fn read_file(filename: &str) -> Result<String, Error> {
    let mut file_handle = File::open(filename)?;
    let mut file_data = String::new();

    file_handle.read_to_string(&mut file_data)?;
    Ok(file_data)
}

// fn panic_vector() {
//     let vector = vec![1, 2, 3, 4, 5];
//     println!("{}", vector[10]);
// }
