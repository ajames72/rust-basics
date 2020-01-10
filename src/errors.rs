pub mod error_handling {
    #[allow(dead_code)]
    pub fn my_panic() {
        panic!("crash and burn");
    }

    // Matches in Rust are exhaustive:
    // we must exhaust every last possibility in order for the code to be valid.
    pub fn controlling_flow(opt: u8) {
        match opt {
            1 => {
                println!("opt param matches 1");
            },
            2 => {
                println!("opt param matches  2");
            },
            3 => {
                println!("opt param matches  3");
            },
            4 => {
                println!("opt param matches  4");
            },
            5 => {
                println!("opt param matches  5");
            },
            // The _ pattern will match any value
            _ => (),
        }
    }

    // pub fn throw_error(x: Option<u8>) -> Result<u8, &'static str> {
    //     match x {
    //         Some(val) => Ok(val),
    //         None => Err("No parameter was passed"),
    //     }
    // }
    
    fn check_value(opt: u8) -> Result<u8, &'static str> {
        match opt {
            // 1 => Ok(1),
            // 2 => Ok(2),
            // 3 => Ok(3),
            // 4 => Ok(4),
            // 5 => Ok(5),
            1..=5 => Ok(opt),
            _ => Err("Wrong Value --> {}"),
        }
    }

    pub fn throw_error(opt: u8) {
        let r = check_value(opt);
        match r {
            Ok(r) => {
                println!("will not throw errow --> {}", r);
            },
            Err(error) => {
                println!("this is an error {}", error);
            }
        };
    }
}