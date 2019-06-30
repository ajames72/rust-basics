pub mod literals {
    pub fn integer_literal_test() {
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

    pub fn numeric_operations_test() {
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

    pub fn character_test() {
        // char literals are specified with single quotes.
        // string literals use double quotes.
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';
        println!("Value of characters are {}, {}, {}", c, z, heart_eyed_cat);
    }

    pub fn compound_type_test() {
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
}

pub mod functions {
    pub fn parameter_test(param: i32) {
        println!("value of function parameter is {}", param);
    }

    pub fn scope_test() {
        let x = 7; // generates a warning 'help: consider using `_x` instead'

        let y = {
            let x = 3;
            // Note: no semi colon terminator
            x + 1
        };

        println!("Value of Y is {}", y);
    }

    pub fn return_value_test() {
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
}

pub mod scope {
    // passing a variable to a function will either move or copy a variable
    pub fn ownership_and_functions() {
        let s = String::from("string var to be moved as function param");  // s comes into scope

        takes_ownership(s);             // s's value moves into the function...
                                        // ... and so is no longer valid here
        // println!("Testing s {}", s); // err - value borrowed here after move

        let x = 5;                      // x comes into scope

        makes_copy(x);                  // x would move into the function,
                                        // but i32 is Copy, so it’s okay to still
                                        // use x afterward
        let s1 = gives_ownership();

        let s2 = String::from("comes into scope");

        let s3 = takes_and_gives_back(s2);
        
        println!("The strings {} {}", s1, s3);
        
        let mut s4 = String::from("The strings ");
        
        // ampersands are references, and they allow you
        // to refer to some value without taking ownership of it
        // s4.push_str(&s1);
        // 
        // s4.push_str(" ");
        // 
        // s4.push_str(&s3);
        
        concat_strings(&mut s4, &s1, &s3);
        // let (s5, len) = calculate_length(s4);
        // 
        // println!("The length of '{}' is {}", s5, len);
        let len = calculate_length(&s4);
        
        println!("The length of '{}' is {}", s4, len);
        
        let len = first_word(&s4);
        
        println!("The length of the first word '{}' is {}", &s4[0..len], len);

    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
      // memory is freed.

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.

    fn gives_ownership() -> String {
        let some_string = String::from("Hello");
        
        some_string
    }

    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }

    fn concat_strings(concat_string: &mut String, str_a: &String, str_b: &String) {
        concat_string.push_str(str_a);
        concat_string.push_str(" ");
        concat_string.push_str(str_b);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for(i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }
}
