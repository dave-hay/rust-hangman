use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::io;

const WORDS: [&str; 3] = ["hi", "hello", "nuts"];

// choose random word
fn get_word() -> String {
    let mut rng = thread_rng();
    let n = rng.gen_range(0..WORDS.len());
    WORDS[n].to_string()
}

fn main() {
    let word = get_word();
    let mut chars_left = word.len();
    let mut word_map = HashMap::new();

    // build map {char: count}
    for c in word.chars() {
        if word_map.contains_key(&c) {
            if let Some(x) = word_map.get_mut(&c) {
                *x += &1
            }
        } else {
            word_map.insert(c, 1);
        }
    }
    // dbg!(&word_map);

    let mut turns = 0;

    while turns < 5 {
        // get users guess
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read");
        let guess = guess.trim().to_owned();

        for c in guess.chars() {
            if word_map.contains_key(&c) {
                if let Some(x) = word_map.get_mut(&c) {
                    *x -= &1;
                    chars_left -= 1;
                }
            }
        }
        // dbg!(&word_map);

        turns += 1;

        // build correctly guessed map

        // find where there is a match
        // update board
        // if all correct return

        for c in word.chars() {
            println!("{c}");
        }
    }
}
