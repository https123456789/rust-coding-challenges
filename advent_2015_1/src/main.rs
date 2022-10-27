use std::fs;
use std::env;

fn main() {
    println!("Advent of Code 2015\nDay 1: Not Quite Lisp");

    // Get args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("\x1b[31mError: Invalid args.\x1b[0m");
        return;
    }
    
    // Read the input
    let contents = fs::read_to_string(&args[1])
        .expect("Should have been able to read the file");
    
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