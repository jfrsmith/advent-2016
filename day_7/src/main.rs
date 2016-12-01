use std::io::{BufReader,BufRead};
use std::fs::File;
use std::collections::HashMap;

struct Instruction {
    input: String,
    output: String
}

fn parse_instruction(raw_instruction: &str) -> Instruction {
    let v: Vec<_> = raw_instruction.split("->").collect();
    Instruction { input: v[0].to_string(), output: v[1].to_string()}
}

fn to_int(symbol: &str, wires: &HashMap<String, u16>) -> Option<u16> {
    match symbol.parse() {
        Ok(num) => return Some(num),
        Err(_) => match wires.get(symbol) {
        	Some(num) => return Some(*num),
        	None	=> return None
        }
    }
}

fn resolve_input(instr: &Instruction, wires: &HashMap<String, u16>) -> Option<u16> {
    let v: Vec<_> = instr.input.split_whitespace().collect();
    match v.len() {
        1 => return to_int(v[0], &wires),
        2 => match to_int(v[1], &wires) {
        	Some(num) => return Some(!num),
        	None => return None,
        },
        3 => if let Some(lhs) = to_int(v[0], &wires) {
        	    if let Some(rhs) = to_int(v[2], &wires) {
        	    	match v[1] {
	        	    	"AND" => return Some(lhs & rhs),
			            "OR" => return Some(lhs | rhs),
			            "LSHIFT" => return Some(lhs << rhs),
			            "RSHIFT" => return Some(lhs >> rhs),
			            _ => panic!("Unexpected instruction: {}", v[1])
	        		}
        	    }
        	},
        _ => panic!("Unexpected number of inputs {}", v.len())
    }

    None
}

fn main() {
	let mut wires = HashMap::new();
	let mut instructions = Vec::new();

    let file = File::open("input/input.txt").unwrap();
    for line in BufReader::new(file).lines() {
    	instructions.push(parse_instruction(&line.unwrap()));
    }

    while let None = wires.get("a") {
    	for instr in &instructions {
	        if let Some(output) = resolve_input(&instr, &wires) {
	            wires.insert(instr.output.trim().to_string(), output);
	        }
    	}
    }

    println!("a = {}", wires.get("a").unwrap());

    let mut index = 0;
    for (i, instr) in instructions.iter().enumerate() {
        if instr.output.trim() == "b" {
        	index = i;
            println!("{} -> {}", instr.input, instr.output);
            break;
        }
    }

    instructions.remove(index);
    instructions.insert(0, parse_instruction(&format!("{} -> b", wires.get("a").unwrap())));

    wires.clear();

    while let None = wires.get("a") {
    	for instr in &instructions {
	        if let Some(output) = resolve_input(&instr, &wires) {
	            wires.insert(instr.output.trim().to_string(), output);
	        }
    	}
    }

    println!("a = {}", wires.get("a").unwrap());
    
}