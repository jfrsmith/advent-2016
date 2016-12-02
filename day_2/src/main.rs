fn clamp(index: i32, min : i32, max: i32) -> i32 {
	std::cmp::max(std::cmp::min(index, max), min)
}

fn to_digit(key: &(i32,i32), keypad : &Vec<Vec<char>>) -> char {
	keypad[key.1 as usize][key.0 as usize]
}

fn move_key(from: &(i32,i32), dir: &(i32,i32), keypad : &Vec<Vec<char>>) -> (i32,i32) {
	let new_loc = (clamp((from.0 + dir.0), 0, keypad.len() as i32 - 1), clamp((from.1+dir.1), 0, keypad.len() as i32 - 1));
	match to_digit(&new_loc, keypad) {
		'-' => (from.0, from.1),
		_	=> new_loc
	}
}

fn to_direction(from: &char) -> (i32,i32) {
	match from {
		&'U' => (0,-1),
		&'L' => (-1,0),
		&'D' => (0,1),
		&'R' => (1,0),
		_   => panic!("Invalid direction letter: {}", from)
	}
}

fn get_bathroom_code(input_str: &str, keypad : &Vec<Vec<char>>, starting_position : (i32, i32)) -> String {
	input_str.lines().fold(vec![starting_position], | keys, line | {
		let pressed_key = line.chars().fold(*keys.last().unwrap(), | current_key, input | {
			move_key(&current_key, &to_direction(&input), &keypad)
		});
		keys.iter().chain(vec![pressed_key].iter()).cloned().collect()
	}).split_at(1).1.iter().map(|x| {
		to_digit(x, &keypad)
	}).collect()
}

fn get_part_one_keypad() -> Vec<Vec<char>> {
	vec![
		vec!['1','2','3'],
		vec!['4','5','6'],
		vec!['7','8','9']
		]
}

fn get_part_two_keypad() -> Vec<Vec<char>> {
	vec![
		vec!['-','-','1','-','-'],
		vec!['-','2','3','4','-'],
		vec!['5','6','7','8','9'],
		vec!['-','A','B','C','-'],
		vec!['-','-','D','-','-'],
		]
}

fn main() {
	println!("Part one code: {:?}", get_bathroom_code(include_str!("../input/input.txt"), &get_part_one_keypad(), (1,1)));
	println!("Part wto code: {:?}", get_bathroom_code(include_str!("../input/input.txt"), &get_part_two_keypad(), (0,2)));
}

#[test]
fn part_one() {
    let inputs = "ULL
RRDDD
LURDL
UUUUD";
    assert_eq!("1985", get_bathroom_code(&inputs, &get_part_one_keypad(), (1,1)));
}

#[test]
fn part_two() {
    let inputs = "ULL
RRDDD
LURDL
UUUUD";
    assert_eq!("5DB3", get_bathroom_code(&inputs, &get_part_two_keypad(), (0,2)));
}