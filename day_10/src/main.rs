fn main() {
    println!("Output = {}", calculate_look_say("3113322113", 50).len());
}

fn calculate_look_say(input_num: &str, num_iterations: u32) -> String {
	let mut new_val = input_num.to_string();

    if num_iterations > 0 {
        new_val = calculate_look_say(&iterate_look_say(input_num), num_iterations - 1);
    }

    new_val
}

fn iterate_look_say(input_num: &str) -> String {

	let mut output = String::new();
	let mut v: Vec<char> = input_num.chars().collect();
	let mut last_num = v[0];
	let mut num_occurences = 1;
	v.remove(0);

	for c in v {
	    if c != last_num {
	        output = output + &format!("{}{}", num_occurences, last_num);
	        last_num = c;
	        num_occurences = 1;
	    }
	    else {
	        num_occurences += 1;
	    }
	}

	output = format!("{}{}{}", output, num_occurences, last_num);
	
	output
}