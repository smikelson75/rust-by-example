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

    // Lifetimes
    // Dangling reference - inner_scope is deallocated leaving outer_scope
    // pointing to a memory address that doesn't have anything
    //let outer_scope;

    // {
    //     let inner_scope = 5;
    //     outer_scope = &inner_scope;
    // }

    // println!("{}", outer_scope);

    // Lifetime syntax
    // lifetimes apply only to reference variables (p1, p3) and not value (p2)
    // fn lifetime_syntax<a, b>(p1: &'a i32, p2: i32, p3: &b' f64)

    let value_one = 24;
    let value_two = 67;

    let value = explict_lifetime(&value_one, &value_two);

    // output: 67
    println!("{}", value);

}

fn explict_lifetime<'a>(p1: &'a i32, p2: &'a i32) -> &'a i32 {
    if p1 > p2 {
        p1
    } else {
        p2
    }
}