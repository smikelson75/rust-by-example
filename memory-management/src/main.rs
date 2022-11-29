fn main() {
    // In Rust, there can be 1 and only 1 owner
    // of data at a time.
    let mut original = String::from("original value");
    println!("\noriginal: \t\"{}\"\n", original);

    //let next_ownership = original;
    // Ownership of the value stored in original is now owned by next_ownership.
    // https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html


    //let next_borrow = &original;
    // The value of original is borrowed by next_borrow, and since, the value of
    // original is immutible, the reference contained by next_borrow is read only.


    //original = String::from("new value");
    // When data is borrowed from original to next_borrow and the value
    // of original is change, rust can no longer guarentee memory saftey
    // and the above code, is will not compile when original is borrowed.

    {
        // original will only regain control of the value
        // once next falls out of scope. The only way to do 
        // this is to put the assignment in curly braces
        let next = &original;

        println!("inner scope next \t\"{}\"", next);
        println!("outer scope next \t\"{}\"", original);
    }

    // next is now out of scope. value ownership returns to original
    println!("{}", original);

    {
        // the reference stored in next is now mutable
        let next = &mut original;

        // to change the value, de-reference next and store a new value
        *next = String::from("next value");

        println!("inner scope next \t\"{}\"", next);
        println!("outer scope next \t\"{}\"", original);
    }

    // the value now referenced by both next AND original is "new value"
    println!("{}", original);

}
