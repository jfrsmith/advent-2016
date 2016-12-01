use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::open("input/input.txt").unwrap();
    let mut input_str = String::new();
    file.read_to_string(&mut input_str).unwrap();

    let mut floor = 0;
    let mut instructions = 0;
    let mut first_basement_entry = -1;
    for c in input_str.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
			_ => {}
        }

        instructions += 1;

        if first_basement_entry == -1 && floor == -1 {
            first_basement_entry = instructions;
        }
    }

    println!("floor = {} first_basement_entry = {}", floor, first_basement_entry);
}
