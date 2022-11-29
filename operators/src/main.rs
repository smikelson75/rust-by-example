#![allow(unused_variables)]

fn main() {
    // Modulus operator
    // output: 4
    let modulus = 18 % 7;
    println!("{}", modulus);

    // Raise an integer to an exponent of integer
    // output: 64
    let squared = i32::pow(8, 2);
    println!("{}", squared);

    // Raise a float to an exponent of an integer
    // output: 274.625
    let float_integer_exponent = f32::powi(6.4, 3);
    println!("{}", float_integer_exponent);

    // Raise a float to an exponent of a float
    // output: 356.9012
    let float_float_exponent = f32::powf(6.5, 3.14);
    println!("{}", float_integer_exponent);

    // Order of operations (Parenthesis, Exponents, Multiplication/Division, Addition/Subtraction)
    // Execute order of operation, when not significant, then left to right, 
    // output: 9
    let order_ops = 8+4*2-(12/3+7)+4;
    println!("{}", order_ops);

    // value: true
    let are_equal_is_true = 1 == 1;
    // value: false
    let are_equal_is_false = 1 == 2;
    // value: true
    let are_not_equal = 1 != 2;

    let is_true = true;
    let is_false = !is_true;

    // output: is_true = true; is_false = false
    println!("is_true = {}; is_false = {}", is_true, is_false);

    // Logical And (&&)
    // value: true
    let logic_and_is_true = true && true;
    // value: false
    let logic_and_is_false = true && false;
    // value: false 
    let logic_and_is_false_2 = false && true;
    // value: false
    let logic_and_is_false_3 = false && false;

    // Logical OR (||)
    // value: true
    let logic_or_is_true = true || true;
    // value: true
    let logic_or_is_true_2 = true || false;
    // value: true
    let logic_or_is_true_3 = false || true;
    // value: false
    let logic_or_is_false = false || false;

    // Bitwise Operators (restricted to integers)
    // And (&)
    // 0101 0110
    // 0001 1011 &
    // -----------
    // 0001 0010
    // value: 18
    let bitwise_and = 86 & 27;

    // Or (|)
    // 0101 0110
    // 0001 1011 |
    // -----------
    // 0101 1111
    // value: 95
    let bitwise_or = 86 | 27;

    // XOr (^)
    // 0101 0110
    // 0001 1011 ^
    // -----------
    // 0100 1101
    // value: 77
    let bitwise_xor = 86 | 27;

    // Left shift (<<)
    // 0101 0110 << by 1 bit is 1010 1100 
    // value: 172
    let bitwise_left_shift = 86 << 1;

    // Right shift (>>)
    // 0101 0110 >> by 1 bit is 0010 1011
    // value: 43
    let bitwise_right_shift = 86 >> 1;

}
