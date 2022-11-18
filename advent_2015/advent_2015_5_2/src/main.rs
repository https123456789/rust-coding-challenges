use shared_lib::{get_args, get_contents_of_file};

fn main() {
    println!("Advent of Code 2015\nDay 5 Part 2: Doesn't He Have Intern-Elves For This?");

    // Read input
    let input = get_contents_of_file(get_args(2)[1].clone());

    let lines: Vec<&str> = input.as_str().split("\n").collect();

    let mut nice_count = 0;
    
    for line in lines.clone() {
        if is_nice_word(&line) {
            nice_count += 1;
        }
    }

    println!("Nice Words: {} out of {}", nice_count, lines.clone().len());
}

fn is_nice_word(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let clen = chars.clone().len();

    // Requirement: A set of 2 characters that occurs 2 times and not overlapping
    let mut is_tan_ov = false;
    for (outer_index, _outer_char) in chars.iter().enumerate() {
        // Break from this loop if we know that we have already met the reaquirement
        if is_tan_ov {
            break;
        }
        // 2 Loops: 2 pairs -- compare
        if outer_index + 1 < clen {
            let cpair: [&char; 2] = [ &chars[outer_index], &chars[outer_index + 1] ];
            for (inner_index, _inner_char) in chars.iter().enumerate() {
                // Skip if there is an overlap
                if inner_index == outer_index || inner_index == outer_index + 1 || (outer_index > 0 && inner_index == outer_index - 1) {
                    continue;
                }
                if inner_index + 1 < clen {
                    let cpair2: [&char; 2] = [ &chars[inner_index], &chars[inner_index + 1] ];
                    if cpair[0] == cpair2[0] && cpair[1] == cpair2[1] {
                        is_tan_ov = true;
                        break;
                    }
                }
            }
        }
    }

    // Requirement: At least one character that appears again after a middle char
    let mut is_t = false;
    for (i, c) in chars.iter().enumerate() {
        // If possible, check if the next char is the same
        if i + 2 < clen {
            if c.clone() == chars.clone()[i + 2] {
                is_t = true;
                break;
            }
        }
    }

    return is_t && is_tan_ov;
}