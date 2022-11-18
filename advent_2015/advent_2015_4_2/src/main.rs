mod md5;

use shared_lib::{get_args, get_contents_of_file};
use md5::md5_utf8;
use std::io::{self, Write};

fn main() {
    println!("Advent of Code 2015\nDay 4 Part 2: The Ideal Stocking Stuffer");

    // Read input
    let contents = get_contents_of_file(get_args(2)[1].clone());

    let mut hash: String;
    let mut num: u32 = 0;

    print!("Working...");
    io::stdout().flush().expect("Unable to flush stdout");
    
    loop {
        let m = format!("{}{}", contents, num);
        hash = md5_utf8(m.as_str());
        if num % 1_003 == 0 {
            print!("\rCurrent: {} {}", num, hash);
            io::stdout().flush().expect("Unable to flush stdout");
        }
        if &hash.as_str()[0..6] == "000000" {
            break;
        }
        num += 1;
    }
    println!("\rFound hash: {:?}", hash);
    println!("So the answer is {}", num);
}
