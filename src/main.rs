use std::io;

fn main() {
    mut_test();
    const_test();
    spacing_test();
    integer_literal_test();
    numeric_operations_test();
    character_test();
    compound_type_test();
    parameter_test(5);
    scope_test();
    return_value_test();
    println!("Guess the number!");
    println!("Please input the guess!");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to real line.");

    println!("You guessed {}", guess);
}

fn mut_test() {
    // Creating a variable and assigning a value to it with the let keyword is a statement
    let mut x = 5;
    println!("Value of x is {}", x);
    x = 6;
    println!("Value of x is {}", x);
}

fn const_test() {
    const MY_CONST: u32 = 100_000;
    println!("Value of MY_CONST {}", MY_CONST);
}

fn spacing_test() {
    let x = 5;
    println!("Value of x is {}", x);
    let x = x + 1;
    println!("Value of x is {}", x);
    let x = x * 2;
    println!("Value of x is {}", x);
    let x = "String Val";
    println!("Value of x is {}", x);
    let x = x.len();
    println!("Value of x is {}", x);
}

fn integer_literal_test() {
    let decimal = 98_445;
    println!("Value of decimal is {}", decimal);
    let hex = 0xFF;
    println!("Value of hex is {}", hex);
    let octal = 0o77;
    println!("Value of octal is {}", octal);
    let binary = 0b1111_0000;
    println!("Value of binary is {}", binary);
    let byte = b'A';
    println!("Value of byte is {}", byte);
}

fn numeric_operations_test() {
    // addition
    let sum = 5 + 10;
    println!("Value of sum is {}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("Value of difference is {}", difference);
    // multiplication
    let product = 4 * 30;
    println!("Value of product is {}", product);
    // division
    let quotient = 56.7 / 32.2;
    println!("Value of quotient is {}", quotient);
    // remainder
    let remainder = 43 % 5;
    println!("Value of remainder is {}", remainder);
}

fn character_test() {
    // char literals are specified with single quotes.
    // string literals use double quotes.
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("Value of characters are {}, {}, {}", c, z, heart_eyed_cat);
}

fn compound_type_test() {
    // A tuple is a general way of grouping together some number
    // of other values with a variety of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow
    // or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("destructure to get values of tuple {}, {}, {}", x, y, z);
    println!("or access tuple directly with '.' {}, {}, {}", tup.0, tup.1, tup.2);
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Value of array is {} {}", array[0], array[1]);
}

fn parameter_test(param: i32) {
    println!("value of function parameter is {}", param);
}

fn scope_test() {
    let x = 7; // generates a warning 'help: consider using `_x` instead'

    let y = {
        let x = 3;
        // Note: no semi colon terminator
        x + 1
    };

    println!("Value of Y is {}", y);
}

fn return_value_test() {
    let x = five();
    println!("The value of five is {}", x);
    let x = plus_one(x);
    println!("The value of plus_one is {}", x);
}

fn five() -> i32 {
    // adding a semicolon changes it from an expression to a statement
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}