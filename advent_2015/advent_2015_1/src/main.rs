use shared_lib::{get_args, get_contents_of_file};

fn main() {
    println!("Advent of Code 2015\nDay 1: Not Quite Lisp");
    
    // Read the input
    let contents = get_contents_of_file(get_args(2)[1].clone());
    
    let mut level = 0;
    let mut basement_at = 0;
    let mut i = 0;

    // Loop over the file content and change the level based on the character
    while i < contents.len() {
        let c = &contents[i..i+1];
        if c.eq("(") {
            level += 1;
        } else {
            level -= 1;
        }
        if level < 0 && basement_at == 0 {
            basement_at = i + 1;
        }
        i += 1;
    }
    println!("Level: {}\nEnters Basement At: {}", level, basement_at);
}