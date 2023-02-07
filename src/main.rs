use rand::{thread_rng, Rng};
use std::io::{self, BufRead};

const WORDS: [&str; 3] = ["hi", "hello", "nuts"];

fn get_word() -> String {
    let mut rng = thread_rng();
    let n = rng.gen_range(0..WORDS.len());
    WORDS[n].to_string()
}

fn main() {
    let w = get_word();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("Hello, {}!", line.unwrap());
    }
}
