use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::io;

const WORDS: &str = include_str!("words.txt");

// choose random word
fn get_word() -> String {
    let w: String = WORDS.to_string();
    let mut words = Vec::new();
    let mut l: usize = 0;

    for (r, ch) in w.chars().enumerate() {
        if ch == '\n' {
            words.push(&w[l..r]);
            l = r + 1;
        }
    }
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(0..words.len());
    words[n].to_string()
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

fn been_guessed(c: &char, guessed: &Vec<char>) -> bool {
    for g in guessed {
        if g == c {
            return true;
        }
    }
    false
}

fn print_vec(v: &Vec<char>) {
    for c in v {
        print!("{} ", c);
    }
    print!("\n");
}

fn main() {
    let word = get_word();
    let mut chars_left = word.len();
    let mut dict = build_dict(&word);
    let mut turns = 6;
    let mut board = vec!['_'; word.len()];
    let mut guessed: Vec<char> = Vec::new();

    println!("Guess a letter!   You have {} guesses", turns);
    while turns > 0 && chars_left > 0 {
        if turns != 6 {
            println!("{} guesses left.", turns);
        }

        print_vec(&guessed);
        print_vec(&board);

        // get users guess
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read");
        let guess = guess.trim().to_owned();

        if guess.len() != 1 {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("You need to guess only one letter!");
            continue;
        }
        let g: char = guess.chars().nth(0).unwrap();

        if been_guessed(&g, &guessed) {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("Already guessed that!");
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
            guessed.push(c);
        }
        // https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed#:~:text=status().-,unwrap()%3B,%22clear%22%20command%20to%20terminal.
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    if chars_left == 0 {
        println!("WINNEEERRRRR!!!!!!!!!!!!!");
    } else {
        println!("😭😭😭😭😭😭😭😭😭😭😭😭😭");
        println!("The word was ~{}~", word);
    }
}
