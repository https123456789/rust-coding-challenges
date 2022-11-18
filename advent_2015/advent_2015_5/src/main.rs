use shared_lib::{get_args, get_contents_of_file};

fn main() {
    println!("Advent of Code 2015\nDay 5: Doesn't He Have Intern-Elves For This?");

    // Read input
    let input = get_contents_of_file(get_args(2)[1].clone());

    let lines: Vec<&str> = input.as_str().split("\n").collect();

    let mut nice_count = 0;
    
    for line in lines {
        if is_nice_word(&line) {
            nice_count += 1;
        }
    }

    println!("Nice Words: {}", nice_count);
}

fn is_nice_word(s: &str) -> bool {
    let mut is_nice: bool = true;
    let valid_vowels = [ 'a', 'e', 'i', 'o', 'u' ];
    let naughty_strings = [ "ab", "cd", "pq", "xy" ];

    let chars: Vec<char> = s.chars().collect();

    // Requirement: At least 3 vowels
    
    let mut vowel_count: u8 = 0;

    for c in chars.clone() {
        for v in valid_vowels {
            if v == c {
                vowel_count += 1;
            }
        }
    }

    is_nice = is_nice && vowel_count > 2;

    // Requirement: A nice string can't have ab, cd, pq, xy

    for ns in naughty_strings {
        if s.contains(ns) {
            is_nice = false;
            break;
        }
    }

    // Requirement: At least one character that appears twice in a row

    let clen = chars.clone().len();
    let mut is_tinar = false;
    for (i, c) in chars.iter().enumerate() {
        // If possible, check if the next char is the same
        if i + 1 < clen {
            if c.clone() == chars.clone()[i + 1] {
                is_tinar = true;
                break;
            }
        }
    }

    is_nice = is_nice && is_tinar;

    return is_nice;
}