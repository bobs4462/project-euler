pub mod utils {
    pub fn input(message: &impl std::fmt::Display) -> String {
        print!("{}", message);
        use std::io::Write;
        std::io::stdout().flush().unwrap();
        let mut ret = String::new();
        std::io::stdin()
            .read_line(&mut ret)
            .expect("Failed to read from stdin");
        ret.trim().to_string()
    }
}

pub mod exercises {
    use std::collections::HashMap;
    pub struct Exercises {
        map: HashMap<&'static str, fn() -> ()>,
    }
    impl Exercises {
        pub fn new() -> Self {
            let mut map = HashMap::with_capacity(500);
            map.insert("exercise_1", super::exercise_1::run as fn() -> ());
            map.insert("exercise_2", super::exercise_2::run as fn() -> ());
            Exercises { map }
        }
        pub fn get(&self, key: &str) -> &fn() -> () {
            self.map.get(key).unwrap()
        }
    }
}

mod exercise_1;
mod exercise_2;
