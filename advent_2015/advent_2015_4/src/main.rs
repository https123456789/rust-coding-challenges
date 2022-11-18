mod md5;

use shared_lib::{get_args, get_contents_of_file};
use md5::md5_utf8;

fn main() {
    println!("Advent of Code 2015\nDay 4: The Ideal Stocking Stuffer");

    // Read input
    let contents = get_contents_of_file(get_args(2)[1].clone());

    let mut hash: String;
    let mut num: i32 = 1;
    
    loop {
        let m = format!("{}{}", contents, num);
        if num % 100 == 0 {
            print!("\rCurrent Number: {}", num);
        }
        hash = md5_utf8(m.as_str());
        let s = &hash.as_str()[0..5];
        if s == "00000" {
            break;
        }
        num += 1;
    }
    println!("\rFound hash: {:?}", hash);
    println!("So the answer is {}", num);
}
