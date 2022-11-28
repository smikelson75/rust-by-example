#![allow(unused_variables)]

fn main() {
    // immutable variable (default)
    let defined_variable: i32 = 0;

    // inferred variable, i32 default
    let inferred_variable = 0;

    // inferred variable, f64 default
    let inferred_variable_float = 1.0;

    // Compiler complains about unused variable, use underscore 
    // when you know it will be unused.
    let _warning_variable = 0;

    let float_thirty_two: f32 = 17.2;
    let unsigned_eight: u8 = 5;

    // Rust doesn't allow for implicit casting. 
    // All casting must be done explicitly
    let cast_unsigned_eight = unsigned_eight as f32;

    let result = float_thirty_two / cast_unsigned_eight;

    // mutable variable
    let mut changeable_variable = 500;

    // primary scope. output: outer_scope
    let scope_test = "outer_scope";
    println!("{}", scope_test);

    {
        // Inner scope, scope_test has been shadowed in the
        // second declaration. The shadowed variable will go out of scope
        // on the closed curly brace (})
        // output: inner_scope
        let scope_test = "inner_scope";
        println!("{}", scope_test);
    }

    // primary scope again. output: outer_scope
    println!("{}", scope_test);

    // variables can be shadowed even in the same scope and the shadowed variable 
    // is not restricted on the type.
    let scope_test: i32 = 0
    println!("{}", scope_test);
}
