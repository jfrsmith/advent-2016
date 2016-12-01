use std::io::{BufReader,BufRead};
use std::fs::File;

fn main() {
	let mut total_code_chars = 0;
	let mut total_encoded_chars = 0;
    let file = File::open("input/input.txt").unwrap();
    for line in BufReader::new(file).lines() {
    	let mut s = line.unwrap().to_string();

    	let curr_code_chars = s.len();
        let mut curr_encoded_chars = s.len() + 2;

        for c in s.chars() {
            match c {
                '\\' => curr_encoded_chars += 1,
                '\"' => curr_encoded_chars += 1,
                _ => {}
            }
        }

        println!("{} -> num_chars = {} num_str = {}", s, curr_code_chars, curr_encoded_chars);

    	total_code_chars += curr_code_chars;
    	total_encoded_chars += curr_encoded_chars;
    }

    println!("total_encoded_chars = {} total_code_chars = {} total = {}", total_encoded_chars, total_code_chars, total_encoded_chars - total_code_chars);
}