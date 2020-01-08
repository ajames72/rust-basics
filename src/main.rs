use std::io;
use std::convert::AsRef;
mod basics;
use basics::literals;
use basics::functions;
use basics::scope;

fn main() {
    mut_test();
    const_test();
    spacing_test();
    set_string_example();
    literals::integer_literal_test();
    literals::numeric_operations_test();
    literals::character_test();
    literals::compound_type_test();
    functions::parameter_test(5);
    functions::scope_test();
    functions::return_value_test();
    scope::ownership_and_functions();
    structs_test();
    let arr: [i32; 5] = return_array();
    println!("Returned array length {}", arr.len());

    println!("Guess the number!");
    println!("Please input the guess!");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to real line.");

    println!("You guessed {}", guess);
    
    input_check(&guess);
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

fn return_array() -> [i32; 5] {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    
    array
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