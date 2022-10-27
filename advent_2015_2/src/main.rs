use std::fs;
use std::env;

mod size;
use size::Size;

fn main() {
    println!("Advent of Code 2015\nDay 2: I Was Told There Would Be No Math");

    // Get args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("\x1b[31mError: Invalid args.\x1b[0m");
        return;
    }
    
    // Read the input
    let contents = fs::read_to_string(&args[1])
        .expect("Should have been able to read the file");
    let split_contents: Vec<&str> = contents.as_str().split('\n').collect();
    let mut total = 0;
    for split in split_contents {
        let size = Size::from_str(split);
        total += size.get_surface_area();
        total += size.get_extra();
    }
    println!("Total: {:?}", total);
}