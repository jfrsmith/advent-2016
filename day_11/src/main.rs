use std::char;

fn main() {
    let input = "hxbxxyzz";
    let mut input: Vec<_> = input.chars().collect();

    loop {
        if !increment_password(&mut input) || 
        	(test_requirement_one(&input) && test_requirement_two(&input) && test_requirement_three(&input)) {
            break;
        }
    }

    let output = input.iter().cloned().collect::<String>();

   	println!("{}", output);
}

fn increment_password(password: &mut Vec<char>) -> bool {    
    for n in (0..password.len()).rev() {
    	match password[n] {
    	    'z' => password[n] = 'a',
    	    _ => {
    	    	password[n] = char::from_u32(password[n] as u32 + 1).unwrap();
    	    	return true;
    	    },
    	}
    }

    false
}

fn test_requirement_one(input: &Vec<char>) -> bool {
	for n in 0..(input.len() - 2) {
		let n1 = input[n] as u32;
		let n2 = input[n+1] as u32;
		let n3 = input[n+2] as u32;

		if n1 + 1 == n2 && n2 + 1 == n3 {
			//println!("{:?} contains straight {}{}{}", input, input[n], input[n+1], input[n+2]);
			return true;
		}
	}

	false
}

fn test_requirement_two(input: &Vec<char>) -> bool {
    return !input.contains(&'i') && !input.contains(&'o') && !input.contains(&'l');
}

fn test_requirement_three(input: &Vec<char>) -> bool {
	let mut num_pairs = 0;

	for n in 0..(input.len() - 1) {
		if input[n] == input[n+1] && (n == 0 || input[n] != input[n-1]) {
			//println!("{:?} contains pair {}{}", input, input[n], input[n+1]);
			num_pairs += 1;
		}
	}

	num_pairs >= 2
}