extern crate md5;

use std::char;
use std::thread;

fn hex_to_char(hex: u8) -> char {
	char::from_digit(hex as u32, 16).unwrap()
}

fn is_valid_hash(digest: &[u8]) -> bool {
	digest[0] as u32 + digest[1] as u32 + (digest[2] as u32 >> 4) == 0
}

fn get_char_index_from_valid_hash_byte(hash_byte: u8, max_index: usize) -> Option<usize> {
	let index = (hash_byte) as usize;
	match index < max_index {
		true => Some(index),
		false => None
	}
}

fn find_next_unused_hash(door_id: &str, start_index: u32, invalid_indices: &Vec<char>) -> (char, usize, u32) {
	for index in start_index.. {
		let next_door_id = door_id.to_string() + &index.to_string();
		let next_digest = md5::compute(&next_door_id.to_string().into_bytes());

		if is_valid_hash(&next_digest) {
			let placement = get_char_index_from_valid_hash_byte(&next_digest[2] & 0x0f, invalid_indices.len());
			if placement.is_some() && invalid_indices[placement.unwrap()] == ' ' {
				return (hex_to_char(next_digest[3] >> 4), placement.unwrap(), index);
			}
		}
	}

	panic!("Should not get here!");
}

fn find_next_hash(door_id: &str, start_index: u32) -> (char, u32) {
	for index in start_index.. {
		let next_door_id = door_id.to_string() + &index.to_string();
		let next_digest = md5::compute(&next_door_id.to_string().into_bytes());
		
		if is_valid_hash(&next_digest) {
			return (hex_to_char(next_digest[2] & 0x0f), index);
		}
	}

	panic!("Should not get here!");
}

fn get_door_password_part_one(door_id: &str, len: usize) -> String {
	(0..len).scan(0, |start_index, _| {
		let (next_char, found_at) = find_next_hash(door_id, *start_index);
		*start_index = found_at + 1;
		Some(next_char)
	}).collect()
}

fn get_door_password_part_two(door_id: &str, len: usize) -> String {
	(0..len).fold((0, vec!(' ',' ',' ',' ',' ',' ',' ',' ')), | state, _ | {
		println!("{:?}", state);
		let (next_char, placement_index, found_at) = find_next_unused_hash(door_id, state.0, &state.1);
		let mod_vec = (0..len).map(|i| if i == placement_index { next_char } else {' '}).collect::<Vec<char>>();
		(found_at + 1, state.1.iter().zip(mod_vec.iter()).map(|(l,r)| if *r != ' ' { *r } else { *l } ).collect())
	}).1.into_iter().collect()
}

fn main() {

    println!("Door Password (Part one) = {:?}", get_door_password_part_one("ffykfhsq", 8));
    println!("Door Password (Part two) = {:?}", get_door_password_part_two("ffykfhsq", 8));
}

#[test]
fn part_one() {
	assert_eq!(('1', 3231929), find_next_hash("abc", 0));
	assert_eq!(('8', 5017308), find_next_hash("abc", 3231929+1));
	assert_eq!(('f', 5278568), find_next_hash("abc", 5017308+1));
	assert_eq!("18f47a30", get_door_password_part_one("abc", 8));
}

#[test]
fn part_two() {
	assert_eq!(('5', 1, 3231929), find_next_unused_hash("abc", 3231929, &vec!(' ',' ',' ',' ',' ',' ',' ',' ')));
	assert_eq!(('e', 4, 5357525), find_next_unused_hash("abc", 5357525, &vec!(' ',' ',' ',' ',' ',' ',' ',' ')));
	assert_eq!("05ace8e3", get_door_password_part_two("abc", 8));
}