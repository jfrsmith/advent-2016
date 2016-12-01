use std::io::{BufReader,BufRead};
use std::fs::File;

fn is_nice_str(test_str: &str) -> bool {
	//println!("Testing string: {}", &test_str);
	
	let mut last_char = '-';
	let mut vowel_count = 0;
	let mut has_char_repeat = false;
	for c in test_str.chars() {
		has_char_repeat |= c == last_char;
		match c {
		    'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
		    'b' => if last_char == 'a' {
		    	println!("	Failed: Matches naughty pattern 'ab'");
		        return false;
		    },
		    'd' => if last_char == 'c' {
		    	println!("	Failed: Matches naughty pattern 'cd'");
		        return false;
		    },
		    'q' => if last_char == 'p' {
		    	println!("	Failed: Matches naughty pattern 'pq'");
		        return false;
		    },
		    'y' => if last_char == 'x' {
		    	println!("	Failed: Matches naughty pattern 'xy'");
		    	return false;
		    },
		    _ => {}
		}
		last_char = c;
	}

	//println!("	has_char_repeat = {} vowel_count = {}", has_char_repeat, vowel_count);
	vowel_count >= 3 && has_char_repeat
}

fn is_nice_str_v2(test_str: &str) -> bool {
	//println!("Testing string: {}", &test_str);
	
	let mut current_char_minus_one = '-';
	let mut current_char_minus_two = '-';

	let mut has_repeating_pair = false;
	let mut has_char_repeat = false;

	let extended_test_str = format!("{}{}{}", test_str, current_char_minus_one, current_char_minus_two);

	for c in extended_test_str.chars() {
		if !has_char_repeat {
		    //println!("	test {} against {}", c, current_char_minus_two);
		    if c == current_char_minus_two {
		        has_char_repeat = true;
		        //println!("		'{}' matches '{}' and has a gap between them", c, current_char_minus_two);
		    }
		}		

		if !has_repeating_pair {
		    let s = format!("{}{}", current_char_minus_two, current_char_minus_one);
		    //println!("	test '{}'", &s);
			let v: Vec<(usize, &str)> = test_str.match_indices(&s).collect();

			if v.len() > 1 {
				for i in 0..(v.len() - 1) {
				    let (start1, str1) = v[i];
				    let (start2, _) = v[i+1];
				    //println!("	Found matching pair '{}' starting at '{}'", &str1, start2);

				    if start2 >= (start1 + str1.len()) {
				    	//println!("		Has repeating pair '{}' starting at {} and {}", &s, start1, start2);
				    	has_repeating_pair = true;
				    	break;
				    }
				}	
			}
		}	

		current_char_minus_two = current_char_minus_one;
		current_char_minus_one = c;

		if has_char_repeat && has_repeating_pair {
		    break;
		}
	}

	//println!("	has_char_repeat = {} has_repeating_pair = {}", has_char_repeat, has_repeating_pair);
	has_char_repeat && has_repeating_pair
}

fn main() {
	let mut nice_strings = 0;
    let file = File::open("input/input.txt").unwrap();
    for line in BufReader::new(file).lines() {
    	if is_nice_str_v2(&line.unwrap()) {
    	    nice_strings += 1;
    	}
    }

    println!("Nice strings: {}", nice_strings);
}