extern crate rand;

use rand::Rng;

pub struct Robot(String);

impl Robot {
    pub fn new() -> Robot {
        Robot(Robot::generate_random_name())
    }

    pub fn name<'a>(&'a self) -> &'a str {
        self.0.as_str()
    }

    pub fn reset_name(&mut self) {
        self.0 = Robot::generate_random_name()
    }

    fn generate_random_name() -> String {
        let ascii_upper: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        let ascii_digits: Vec<char> = "0123456789".chars().collect();

        let mut rng = rand::thread_rng();
        let mut tmp = String::with_capacity(5);
        // unwrap cannot fail, because vectors are non-empty
        tmp.push(*rng.choose(&ascii_upper).unwrap());
        tmp.push(*rng.choose(&ascii_upper).unwrap());
        tmp.push(*rng.choose(&ascii_digits).unwrap());
        tmp.push(*rng.choose(&ascii_digits).unwrap());
        tmp.push(*rng.choose(&ascii_digits).unwrap());

        tmp
    }
}
