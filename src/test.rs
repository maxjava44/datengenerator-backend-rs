
    pub struct Test {
        pub test : String
    }

    impl Test {
        pub fn new() -> Self {
            return Self{ test: "lul".to_owned() }
        }

        pub fn print(&self) {
            println!("{}",&self.test);
        }
    }
