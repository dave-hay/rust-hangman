use rand::Rng;
use std::io::{self, BufRead};

const WORDS: [&str; 3] = ["hi", "hello", "nuts"];

fn main() {
    let mut rng = rand::thread_rng();
    for word in WORDS {
        println!("{}", word)
    }
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("Hello, {}!", line.unwrap());
    }
}
