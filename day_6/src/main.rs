use std::io::{BufReader,BufRead};
use std::fs::File;
use std::cmp;

enum Command {
    Toggle,
    TurnOff,
    TurnOn,
}

struct Instruction {
    start: (i32, i32),
    end: (i32, i32), 
    command: Command
}

fn parse_instruction(raw_instruction: &str) -> Instruction {
    let mut cmd = Command::Toggle;
	let mut dims_start = Vec::new();
	let mut dims_end = Vec::new();

	let v: Vec<_> = raw_instruction.split_whitespace().collect();
	if v[0] == "turn" {
	    cmd = if v[1] == "off" { Command::TurnOff } else { Command::TurnOn };
	    dims_start = v[2].split(",").collect();
	    dims_end = v[4].split(",").collect();
	}
	else {
	    dims_start = v[1].split(",").collect();
	    dims_end = v[3].split(",").collect();
	}

	Instruction { 	start: (dims_start[0].parse().unwrap(), dims_start[1].parse().unwrap()), 
					end: (dims_end[0].parse().unwrap(), dims_end[1].parse().unwrap()), 
					command: cmd  }
}

fn modify_lights(light_map: &mut Vec<i32>, do_instruction: Instruction) -> () {
    for x in do_instruction.start.0..(do_instruction.end.0+1){
        for y in do_instruction.start.1..(do_instruction.end.1+1) {
            let index: usize = ((x*1000)+y) as usize;
            match do_instruction.command {
                Command::Toggle => light_map[index] += 2,
                Command::TurnOn => light_map[index] += 1,
                Command::TurnOff => light_map[index] = cmp::max(light_map[index] - 1, 0)
            }
        }
    }
}

fn main() {
    let mut lights = vec![0; 1000*1000];

    let file = File::open("input/input.txt").unwrap();
    for line in BufReader::new(file).lines() {
    	modify_lights(&mut lights, parse_instruction(&line.unwrap()));
    }

    let mut total_brightness = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            total_brightness += lights[(x * 1000) + y];
        }
    }

    println!("total_brightness = {}", total_brightness);
}