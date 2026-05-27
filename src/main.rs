use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
//use rand::seq::SliceRandom;
use rand::seq::IndexedRandom;
use rand::seq::SliceRandom;
fn main() {
    let mut words_all = String::new();
    let mut words_common = String::new();
    let mut len_as_num = 0;
    loop {
        println!("Pick a word length:");
        let len = user_input();
        len_as_num = len.trim().parse().expect("Not a number");
        let filename_all = format!("/home/KE-Holtz/Projects/wordmorph/wordlists/{}_len_words.txt", len.trim());
        let filename_common = format!("/home/KE-Holtz/Projects/wordmorph/common_wordlists/{}_len_words.txt", len.trim());

        let path_all = Path::new(&filename_all);
        let path_common = Path::new(&filename_common);

        match fs::read_to_string(path_all) {
            Err(_) => {
                println!("Not a valid length!");
                continue;
            }
            Ok(str) => {
                words_all = str;
            }
        }

        match fs::read_to_string(path_common) {
            Err(_) => {
                println!("Not a valid length!");
                continue;
            }
            Ok(str) => {
                words_common = str;
            }
        }
        break;
    }

    let target_diff: u32 = loop {
        println!("Enter a difficulty (number between 0 and {len_as_num})");
        match user_input().trim().parse() {
            Err(_) => println!("Enter a valid number!"),
            Ok(num) => break if num < len_as_num { num } else { len_as_num },
        }
    };

    let words_all: Vec<&str> = words_all.split("\n").collect();
    let mut words_common: Vec<&str> = words_common.split("\n").collect();
    let mut used_in_solution: Vec<&str> = Vec::new();
    //let mut invalid_starters: Vec<&str> = Vec::new();
    let mut start_word = *words_common
        .choose(&mut rand::rng())
        .expect("Something bad happened :(");
    let mut current_word = start_word.to_string();

    words_common.shuffle(&mut rand::rng());
    let end_word = loop {
        for word in &words_common {
            if diff_of(&current_word, word) == 1
                && diff_of(start_word, word) >= diff_of(start_word, &current_word)
                && !used_in_solution.contains(&word)
            {
                //println!("Using valid word {word}");
                used_in_solution.push(word);
                current_word = word.to_string();
            }
        }
        if diff_of(&current_word, start_word) < target_diff {
            /*println!(
                "Did not make the cut. Target diff was {target_diff}, current diff is {}",
                diff_of(&current_word, start_word)
            );*/
            start_word = *words_common
                .choose(&mut rand::rng())
                .expect("Something bad happened :(");
            current_word = start_word.to_string();
            //println!("Trying a new random start: {start_word}");
        } else {
            break current_word.clone();
        }
    };
    //println!("{start_word}");
    println!("Your start word is: {}", start_word);
    println!("Your end word is: {}", end_word);
    current_word = start_word.to_string();
    let mut attempts = 0;
    loop {
        let guess = user_input().trim().to_string();
        if guess.len() != current_word.len() || !words_all.contains(&guess.as_str()) {
            println!("Not a valid word! (continue from {current_word})");
            continue;
        }
        if diff_of(&guess, &current_word) != 1 {
            println!("Your word can only differ by 1 letter!");
            continue;
        }
        current_word = guess;
        attempts += 1;
        if current_word == end_word {
            println!("You won in {} attempts!", attempts);
            break;
        }
    }
}

fn user_input() -> String {
    print!(">");
    let _ = io::stdout().flush();

    let mut string = String::new();
    let _ = io::stdin().read_line(&mut string);
    string
}

fn diff_of(first: &str, second: &str) -> u32 {
    first
        .chars()
        .zip(second.chars())
        .map(|(a, b)| if a == b { 0 } else { 1 })
        .sum()
}
