use std::thread;

// Thread messaging
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

// https://rust-lang-nursery.github.io/rust-cookbook/concurrency.html
// https://doc.rust-lang.org/book/ch16-00-concurrency.html
fn main() {
    let outer_scope = 412;

    // Spawn a thread and transfer ownership of outer_scope
    // to thread using move
    let join_handler = thread::spawn(move || {
        outer_scope * 2
    });

    // Wait for the result to come back from the thread
    // and assigned Result to result
    let result = join_handler.join();

    // Determine if a result was returned or an error
    match result {
        Ok(value) => {
            println!("{}", value);
        }
        Err(_) => {
            println!("Something happened!");
        }
    }

    // When program exits, all threads end regardless of whether they
    // have completed or not.

    // Communication between two process using messanging
    // Create channels for each of the processes
    let (john_tx, john_rx) = mpsc::channel();
    let (sarah_tx, sarah_rx) = mpsc::channel();

    // Create thread and call corresponding messaging function
    // for john.
    let john_handler = thread::spawn(move || {
        john_chat(sarah_tx, john_rx);
    });

    // Create thread and call corresponding messaging function
    // for sarah.
    let sarah_handler = thread::spawn(move || {
        sarah_chat(john_tx, sarah_rx);
    });

    // Run Process, blocking until thread completes.
    match john_handler.join() {
        Ok(_) => {}
        Err(_) => {}
    }

    // Run Process, blocking until thread completes.
    match sarah_handler.join() {
        Ok(_) => {}
        Err(_) => {}
    }

}

// Sarah's chat function used to communicate with John
fn sarah_chat(john_tx: Sender<&str>, sarah_rx: Receiver<&str>) {
    let result = sarah_rx.recv();
    println!("{}", result.unwrap());

    let _send_result = john_tx.send("Hello John!");
}

// John's chat function used to communicate with Sarah
fn john_chat(sarah_tx: Sender<&str>, john_rx: Receiver<&str>) {
    let _send_result = sarah_tx.send("Hello Sarah!");

    let result = john_rx.recv();
    println!("{}", result.unwrap());
}
