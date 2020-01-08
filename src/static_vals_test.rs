pub mod static_values {
    pub struct MyValues {
        pub value_a: &'static str,
        pub value_b: &'static str,
    }

    pub static MY_TYPE: MyValues = MyValues {
        value_a: "Lorem ipsum dolor sit amet, consectetur adipiscing elit",
        value_b: "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
    };
}