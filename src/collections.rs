pub mod vectors {
    // let v: Vec<i32> = Vec::new();
    // Convienience macro vec!
    // let v = vec![1, 2, 3];
    fn get_vector() -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        v.push(100);
        v.push(53);
        v.push(607);
        v.push(31);
        v.push(13);
        v
    }

    pub fn iterate_over_values_in_a_vector() {
        // make it mutable if you want to update after it has been returned
        let mut my_vec = get_vector();
        my_vec.push(11);

        for i in &my_vec {
            println!("{}", i);
        }
    }
}

pub mod arrays {
    fn return_array() -> [i32; 5] {
        let array: [i32; 5] = [1, 2, 3, 4, 5];
        
        array
    }

    pub fn list_array() {
        let arr: [i32; 5] = return_array();
        println!("Returned array length {}", arr.len());
    }
}