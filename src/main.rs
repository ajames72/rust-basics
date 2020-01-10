extern crate ansi_term;
use std::io;
use std::convert::AsRef;
use ansi_term::Colour;
use ansi_term::Style;
mod basics;
mod static_vals_test;
mod collections;
mod errors;
use basics::literals;
use basics::functions;
use basics::scope;
use static_vals_test::static_values;
use collections::vectors;
use collections::arrays;
use errors::error_handling;

fn main() {
    let heading_style = Style::new().italic();
    print!("{}[2J", 27 as char);
    println!("{}", Colour::Yellow.bold().paint("---------------------"));
    println!("{}", Colour::Yellow.bold().paint("---- Rust Basics ----"));
    println!("{}", Colour::Yellow.bold().paint("---------------------"));
    println!("{}", heading_style.paint("\n\nCommon Programming Concepts:\n---------------------------"));
    mut_test();
    const_test();
    spacing_test();
    set_string_example();
    println!("{}", heading_style.paint("\n\nLiterals:\n------------"));
    literals::integer_literal_test();
    literals::numeric_operations_test();
    literals::character_test();
    literals::compound_type_test();
    println!("{}", heading_style.paint("\n\nFunctions:\n-----------"));
    functions::parameter_test(5);
    functions::scope_test();
    functions::return_value_test();
    println!("{}", heading_style.paint("\n\nScope and Ownership:\n-------------------"));
    scope::ownership_and_functions();
    structs_test();
    println!("{}", heading_style.paint("\n\nCollections:\n--------------"));
    vectors::iterate_over_values_in_a_vector();
    arrays::list_array();
    println!("{}", heading_style.paint("\n\nStatic Values:\n---------------"));
    static_vals_test_fn();

    println!("{}", heading_style.paint("\n\nError handling:\n------------------"));
    error_handling::controlling_flow(2);

    // error_handling::throw_error(2);
    // error_handling::throw_error(100);
    println!("{}", Colour::Yellow.bold().paint("\n-----------------------------"));
    println!("{}", Colour::Yellow.bold().paint("---- Error Handling Game ----"));
    println!("{}", Colour::Yellow.bold().paint("-----------------------------"));

    println!("\n{}", Colour::Blue.bold().paint("Guess the number! Please input your guess..."));

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to real line.");

    println!("{} {}", Colour::Green.paint("You guessed:"), Colour::Green.paint(&guess));

    let guess = match u8::from_str_radix(&guess.trim(), 10) {
        Ok(num) => num,
        Err(_) => {
            panic!("Cannot parse value");
        }
    };

    // let guess: u8 = match guess.trim().parse() {
    //         // parse will return an 'ok' value and Ok will return the number.
    //         Ok(num) => num,
    //         // If it cannot parse, the Err pattern is matched ('_' is a catch-all pattern)
    //         Err(_) => {
    //             panic!("Cannot parse value");
    //         },
    //     };
    error_handling::throw_error(guess);
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

    // 'static lifetime elision
    // https://doc.rust-lang.org/reference/lifetime-elision.html#static-lifetime-elision
    const STRING_CONST: &'static str = "rust basics";
    println!("String const {}", STRING_CONST);
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

fn set_string_example() {
    // (::) is an operator that allows us to namespace
    // this particular from function under the String
    // type rather than using some sort of name like 'string_from'
    let mut s  = String::from("hello");
    
    s.push_str(", world!");
    
    println!("{}", s);
}

fn structs_test() {
    let rect1 = Rectangle { width: 30, height: 50};

    println!(
        "The area of the rectangle dimens {:?} is {} square pixels.",
        rect1,
        rect1.area()
    );

    let rect2 = Rectangle { width: 10, height: 40 };

    println!(
        "The area of the rectangle 2 dimens {:?} is {} square pixels. Can rect1 hold rect2? {}",
        rect2,
        rect2.area(),
        rect1.can_hold(&rect2)
    );

    // Associated function
    // To call this associated function, we use the :: syntax with the struct name
    let sq = Rectangle::square(35);

    println!("Can rect2 hold sq? {}", rect2.can_hold(&sq));
}

fn static_vals_test_fn() {
    println!("{}", static_values::MY_TYPE.value_a);
    println!("{}", static_values::MY_TYPE.value_b);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        let this_area = self.width * self.height;
        let rect_area = rectangle.width * rectangle.height;
        if this_area > rect_area {
            return true
        } else {
            return false
        }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

#[allow(dead_code)]
fn input_check(opt: &String) {
    match opt.trim().as_ref() {
        "one" => println!("happy days"),
        _ => println!("Bad!"),
    }
}

// Also
// fn input_check(opt: &String) {
//     match opt.trim() as &str {
//         "one" => println!("happy days"),
//         _ => println!("Bad!"),
//     }
// }