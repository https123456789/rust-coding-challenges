use shared_lib::{get_args, get_contents_of_file};

mod size;
use size::Size;

fn main() {
    println!("Advent of Code 2015\nDay 2: I Was Told There Would Be No Math");

    let contents = get_contents_of_file(get_args(2)[1].clone());
    let split_contents: Vec<&str> = contents.as_str().split('\n').collect();
    let mut total_paper = 0;
    let mut total_ribbon = 0;
    for split in split_contents {
        let size = Size::from_str(split);
        total_paper += size.get_surface_area();
        total_paper += size.get_extra();
        total_ribbon += size.get_ribbon_length();
    }
    println!("Total Paper: {:?} feet", total_paper);
    println!("Total Ribbon: {:?} feet", total_ribbon);
}