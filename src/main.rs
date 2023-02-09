use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::io;

const WORDS: [&str; 3] = ["hi", "hello", "nuts"];

fn get_word() -> String {
    let mut rng = thread_rng();
    let n = rng.gen_range(0..WORDS.len());
    WORDS[n].to_string()
}

fn main() {
    let word = get_word();
    let mut word_map = HashMap::new();

    // build map
    for c in word.chars() {
        if word_map.contains_key(&c) {
            if let Some(x) = word_map.get_mut(&c) {
                *x += &1
            }
        } else {
            word_map.insert(c, 1);
        }
    }

    for (key, val) in word_map.iter() {
        println!("key: {key} val: {val}");
    }

    let mut count = 0;

    while count < 5 {
        // get users guess
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read");
        let guess = guess.trim().to_owned();
        count += 1;

        // find where there is a match
        // update board
        // if all correct return

        for c in word.chars() {
            println!("{c}");
        }
    }
}
