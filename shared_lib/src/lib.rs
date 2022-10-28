use std::fs;
use std::env;
use std::process::exit;

pub fn get_args(required_args_len: usize) -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != required_args_len {
        println!("\x1b[31mError: Invalid args.\x1b[0m");
        exit(1);
    }
    return args;
}

pub fn get_contents_of_file(s: String) -> String {
    return fs::read_to_string(&s)
        .expect("Should have been able to read the file");
}