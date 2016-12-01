use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("input/input.txt").unwrap();
    let mut input_str = String::new();
    file.read_to_string(&mut input_str).unwrap();

    let mut houses = HashMap::new();

    houses.insert((0, 0), 1);

  	let mut santa_location = (0, 0);
  	let mut robo_location = (0, 0);
  	let mut santa_instruction = true;

    for c in input_str.chars() {

    	let (ref mut current_x, ref mut current_y) = *if santa_instruction { &mut santa_location } else { &mut robo_location };

        match c {
            '^' => *current_y += 1,
            'v' => *current_y -= 1,
            '<' => *current_x -= 1,
            '>' => *current_x += 1,
			_ => {}
        }

        houses.insert((*current_x,*current_y), 1);
        santa_instruction = !santa_instruction;
    }

    println!("num houses = {}", houses.len());
}
