extern crate permutohedron;

use std::iter::FromIterator;
use permutohedron::Heap;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn get(instr: &str) -> Direction {
    	match instr {
    		"left" => Direction::Left,
    		"right" => Direction::Right,
    		_ => panic!("Unexpected direction string {:?}", instr)
    	}
    }
}

#[derive(Debug)]
enum Instruction {
    SwapPosition{x: usize, y: usize},
    SwapLetters{a: char, b: char},
    Rotate{x: usize, dir: Direction},
    RotateLetter{a: char},
    Reverse{x: usize, y: usize},
    Move{x: usize, y: usize}
}

fn swap(chars: Vec<char>, x: usize, y: usize) -> Vec<char> {
	let mut copy = chars.to_vec();
	copy.swap(x, y);
	copy
}

fn swap_letters(chars: Vec<char>, a: char, b: char) -> Vec<char> {
	let pos_x = chars.iter().position(|x| *x == a).unwrap();
	let pos_y = chars.iter().position(|x| *x == b).unwrap();
	swap(chars, pos_x, pos_y)
}

fn rotate(chars: Vec<char>, x: usize, dir: Direction) -> Vec<char> {
	let rotator = match dir {
		Direction::Left => x % chars.len(),
		Direction::Right => chars.len() - (x % chars.len())
	};

	chars[..rotator].iter().rev().chain(chars[rotator..].iter().rev()).rev().map(|x| *x).collect()
}

fn rotate_letter(chars: Vec<char>, a: char) -> Vec<char> {
	let pos_char = chars.iter().position(|x| *x == a).unwrap();
	let rotator = pos_char + if pos_char >= 4 {2} else {1};
	rotate(chars, rotator, Direction::Right)
}

fn reverse(chars: Vec<char>, x: usize, y: usize) -> Vec<char> {
	let start_index = std::cmp::max(0, x) as usize;
	let end_index = std::cmp::min(chars.len() - 1, y) as usize + 1;

	chars[..start_index].iter().chain(chars[start_index..end_index].iter().rev().chain(chars[end_index..].iter())).map(|x| *x).collect()
}

fn move_position(mut chars: Vec<char>, x: usize, y: usize) -> Vec<char> {
	let c = chars.remove(x);
	chars.insert(y, c);
	chars
}

fn parse_instruction(line: &str) -> Instruction {
	let split : Vec<&str> = line.split_whitespace().collect();
	match split[0] {
		"swap" => {
			match split[1] {
				"position" => {
					Instruction::SwapPosition{x: split[2].parse::<usize>().unwrap(), 
												y: split[5].parse::<usize>().unwrap()}
				},
				"letter" => {
					Instruction::SwapLetters{a: split[2].chars().nth(0).unwrap(),
												b: split[5].chars().nth(0).unwrap()}
				},
				_ => panic!("Unexpected instruction {:?}", line)
			}
		},
		"reverse" => {
			Instruction::Reverse{x: split[2].parse::<usize>().unwrap(),
									y: split[4].parse::<usize>().unwrap()}
		},
		"rotate" => {
			match split[1] {
				"based" => Instruction::RotateLetter{a: split[6].chars().nth(0).unwrap()},
				"left" | "right" => Instruction::Rotate{dir: Direction::get(split[1]),
														x: split[2].parse::<usize>().unwrap()},
				_ => panic!("Unexpected instruction: {:?}", line)
			}
		},
		"move" => {
			Instruction::Move{x: split[2].parse::<usize>().unwrap(), 
								y: split[5].parse::<usize>().unwrap()}
		},
		_ => panic!("Unexpected instruction: {:?}", line)
	}
}

fn run_instructions(input: &str, starting_string: &str) -> String {
    input.lines().fold(starting_string.to_string().chars().collect(), |char_array, line|{
    	//print!("Input: {:?}", &char_array);
    	let instruction = parse_instruction(line);
    	let chars = match instruction {
		    Instruction::SwapPosition{x, y} => swap(char_array, x, y),
		    Instruction::SwapLetters{a, b} => swap_letters(char_array, a, b),
		    Instruction::Rotate{x, dir} => rotate(char_array, x, dir),
		    Instruction::RotateLetter{a} => rotate_letter(char_array, a),
		    Instruction::Reverse{x, y} => reverse(char_array, x, y),
		    Instruction::Move{x, y} => move_position(char_array, x, y)
    	};

    	//println!(" Instruction {:?}", instruction);

    	chars
    }).into_iter().collect()
}

fn reverse_engineer(desired_output: &str) -> String {
	let instructions = include_str!("../input/input.txt");

	let mut initial : Vec<char> = desired_output.to_string().chars().collect();
	let heap = Heap::new(&mut initial);

	for brute_attempt in heap {
		let output = run_instructions(instructions, &String::from_iter(brute_attempt.to_vec().into_iter()));
		if output == desired_output {
			return brute_attempt.iter().map(|x| *x).collect();
		}
	}

	panic!("Solution not found.");
}

fn main() {
    println!("Scrambled: {:?}", run_instructions(include_str!("../input/input.txt"), "abcdefgh"));
    println!("Unscrambled: {:?}", reverse_engineer("fbgdceah"));
}

#[test]
fn full() {
	let input = "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";

	assert_eq!("decab", run_instructions(input, "abcde"));
}

#[test]
fn swap_test() {
	let chars = "abcde".to_string().chars().collect();
	assert_eq!("ebcda", String::from_iter(swap(chars, 4, 0).into_iter()));
}

#[test]
fn swap_letter_test() {
	let chars = "ebcda".to_string().chars().collect();
	assert_eq!("edcba", String::from_iter(swap_letters(chars, 'd', 'b').into_iter()));
}

#[test]
fn reverse_test() {
	let chars = "edcba".to_string().chars().collect();
	assert_eq!("abcde", String::from_iter(reverse(chars, 0, 4).into_iter()));
}

#[test]
fn rotate_test() {
	let chars = "abcde".to_string().chars().collect();
	assert_eq!("bcdea", String::from_iter(rotate(chars, 1, Direction::Left).into_iter()));
}

#[test]
fn move_test() {
	let mut chars = "bcdea".to_string().chars().collect();
	assert_eq!("bdeac", String::from_iter(move_position(chars, 1, 4).into_iter()));

	chars = "bdeac".to_string().chars().collect();
	assert_eq!("abdec", String::from_iter(move_position(chars, 3, 0).into_iter()));
}

#[test]
fn rotate_letter_test() {
	let mut chars = "abdec".to_string().chars().collect();
	assert_eq!("ecabd", String::from_iter(rotate_letter(chars, 'b').into_iter()));

	chars = "ecabd".to_string().chars().collect();
	assert_eq!("decab", String::from_iter(rotate_letter(chars, 'd').into_iter()));
}