use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::io;

// https://stackoverflow.com/questions/27459640/how-to-create-a-static-array-of-strings/32383866#32383866
const WORDS: &'static [&'static str] = &["hi", "hello", "nuts"];

// choose random word
fn get_word() -> String {
    let mut rng = thread_rng();
    let n = rng.gen_range(0..WORDS.len());
    WORDS[n].to_string()
}

fn build_dict(word: &String) -> HashMap<char, Vec<usize>> {
    let mut dict: HashMap<char, Vec<usize>> = HashMap::new();

    for (i, c) in word.chars().enumerate() {
        // push
        if dict.contains_key(&c) {
            if let Some(x) = dict.get_mut(&c) {
                x.push(i);
            }
        } else {
            // create new vec with ch
            let v = vec![i];
            dict.insert(c, v);
        }
    }
    dict
}

fn main() {
    let word = get_word();
    let mut chars_left = word.len();
    let mut dict = build_dict(&word);
    let mut turns = 5;
    let mut board = vec!['_'; word.len()];

    while turns > 0 && chars_left > 0 {
        println!("Guess a letter!\nYou have {} guesses left.", turns);
        // get users guess
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read");
        let guess = guess.trim().to_owned();

        if guess.len() != 1 {
            println!("You need to guess only one letter!");
            continue;
        }

        // remove from dict and update board values with guess letter
        for c in guess.chars() {
            if dict.contains_key(&c) {
                if let Some((k, v)) = dict.remove_entry(&c) {
                    chars_left -= v.len();
                    for i in v {
                        board[i] = k;
                    }
                }
            } else {
                turns -= 1;
            }
        }

        // print updated board
        let s: String = board.iter().collect();
        println!("{}", s);
    }

    if chars_left == 0 {
        println!("WINNEEERRRRR!!!!!!!!!!!!!");
    } else {
        println!("😭😭😭😭😭😭😭😭😭😭😭😭😭");
    }
}
