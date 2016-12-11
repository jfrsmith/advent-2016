#[derive(Debug, PartialEq)]
enum BotOutputType {
	Bot {bot_number: i32},
	Bin {bin_number: i32}
}

#[derive(Debug, PartialEq)]
struct Input {
	value: i32,
	to_bot: i32
}

#[derive(Debug, PartialEq)]
struct OutputRule {
	low: BotOutputType,
	high: BotOutputType
}

#[derive(Debug, PartialEq)]
struct Bot {
    bot_number: i32,
    outputs: OutputRule,
}

#[derive(Debug, PartialEq)]
struct Rules {
    input_rules: Vec<Input>,
    bot_rules: Vec<Bot>
}

#[derive(Debug, PartialEq)]
struct State {
    bins: Vec<i32>,
    bots: Vec<(i32,i32)>
}

fn find_bot_number(completed_output: &Vec<(i32,i32)>, microchip_pair: (i32,i32)) -> i32 {
	completed_output.iter().position(|&x| {
		(x.0 == microchip_pair.0 || x.0 == microchip_pair.1) &&
		(x.1 == microchip_pair.0 || x.1 == microchip_pair.1)
	}).unwrap() as i32
}

fn get_num_bins(rules: &Rules) -> i32 {
	rules.bot_rules.iter().fold(0, |last_max, bot| {
		let low_bin_number = match bot.outputs.low {
		    BotOutputType::Bin{bin_number} => bin_number,
		    _ => 0
		};

		let high_bin_number = match bot.outputs.high {
			BotOutputType::Bin{bin_number} => bin_number,
			_ => 0
		};

		std::cmp::max(last_max, std::cmp::max(low_bin_number, high_bin_number))
	})
}

fn get_num_bots(rules: &Rules) -> i32 {
	let max_input_rules_bot = rules.input_rules.iter().max_by_key(|input| input.to_bot ).unwrap().to_bot;
	let max_bot_rules_bot = rules.bot_rules.iter().fold(0, |last_max, bot| {
		let low_bot_number = match bot.outputs.low {
		    BotOutputType::Bot{bot_number} => bot_number,
		    _ => 0
		};

		let high_bot_number = match bot.outputs.high {
			BotOutputType::Bot{bot_number} => bot_number,
			_ => 0
		};

		std::cmp::max(last_max, std::cmp::max(low_bot_number, high_bot_number))
	});

	std::cmp::max(max_bot_rules_bot, max_input_rules_bot)
}

fn can_run_rule(token_pair: &(i32,i32)) -> bool {
	token_pair.0 > -1 && token_pair.1 > -1
}

fn bots_have_work_to_do(current_state: &State) -> bool {
	current_state.bots.iter().any(|x| can_run_rule(x))
}

fn take_token_from_bot(token: i32, from_bot: i32, current_bots: &Vec<(i32,i32)>) -> Vec<(i32,i32)> {
	current_bots.iter().enumerate().map(|(i,bot)| {
			if from_bot == i as i32 {
				if bot.0 == token {
					(-1, bot.1)
				}
				else {
				    (bot.0, -1)
				}
			}
			else {
			    *bot
			}
	}).collect()
}

fn deposit_token_in_bin(token: i32, from_bot: i32, to_bin: i32, current_state: &State) -> State {
	State {
		//put the token in the bin
		bins: current_state.bins.iter().enumerate().map(|(i,bin)| {
			if to_bin == i as i32 {
				token
			}
			else {
			    *bin
			}
		}).collect(),
		//take the token off the bot
		bots: take_token_from_bot(token, from_bot, &current_state.bots)
	}
}

fn give_token_to_bot(token: i32, from_bot: i32, to_bot: i32, current_state: &State) -> State {

	let updated_bots = current_state.bots.iter().enumerate().map(|(i,bot)| {
			if to_bot == i as i32 {
				if bot.0 == -1 {
					(token, bot.1)
				}
				else {
				    (bot.0, token)
				}
			}
			else {
			    *bot
			}
		}).collect();

	State {
		//give the token to the new bot
		bins: current_state.bins.to_vec(),
		//take if from the old bot
		bots: take_token_from_bot(token, from_bot, &updated_bots)
	}
}

fn run(input: &str) -> (State, Vec<(i32,i32)>) {
	let rules = build_rules(input);

	let max_bot_number = get_num_bots(&rules) + 1;
	let max_bin_number = get_num_bins(&rules) + 1;

	let mut state = rules.input_rules.iter().fold(State{ bots: vec!((-1,-1); max_bot_number as usize), 
																bins: vec!(0; max_bin_number as usize) }, 
																|current_state, rule| {
		give_token_to_bot(rule.value, -1, rule.to_bot, &current_state)
	});

	let mut bot_compares = vec!((-1,-1); max_bot_number as usize);

	loop {
	    state = rules.bot_rules.iter().fold(state, |current_state, rule| {
			let from_bot_number = rule.bot_number;
			let bot = current_state.bots[from_bot_number as usize];

			if can_run_rule(&bot) {
				bot_compares[from_bot_number as usize] = (bot.0, bot.1);

				let (low, high) = (std::cmp::min(bot.0, bot.1), std::cmp::max(bot.0, bot.1));

				let ref low_rule = rule.outputs.low;
				let ref high_rule = rule.outputs.high;

			    let post_low_rule_state = match low_rule {
			        &BotOutputType::Bot{bot_number} => give_token_to_bot(low, from_bot_number, bot_number, &current_state),
			        &BotOutputType::Bin{bin_number} => deposit_token_in_bin(low, from_bot_number, bin_number, &current_state)
			    };
			    
			    match high_rule {
			        &BotOutputType::Bot{bot_number} => give_token_to_bot(high, from_bot_number, bot_number, &post_low_rule_state),
			        &BotOutputType::Bin{bin_number} => deposit_token_in_bin(high, from_bot_number, bin_number, &post_low_rule_state)
			    }
			}
			else {
			    current_state
			}
		});

	    //short-circuit if bots can't hand out anymore tokens
		if !bots_have_work_to_do(&state) {
			break;
		}
	}

	(state, bot_compares)
}

fn parse_output_rule(output_type: &str, output_num: &str) -> BotOutputType {
	match output_type {
		"bot" => BotOutputType::Bot {bot_number: output_num.to_string().parse().unwrap()},
		"output" => BotOutputType::Bin {bin_number: output_num.to_string().parse().unwrap()},
		_ => panic!("Unrecognised output rule: {:?}", output_type)
	}
}

fn parse_input_rule(instruction: &str) -> Option<Input> {
	let split_instr = instruction.split_whitespace().collect::<Vec<&str>>();
	match split_instr[0] {
	    "value" => Some(Input {
	    				value: split_instr[1].to_string().parse().unwrap(), 
	    				to_bot: split_instr[5].parse().unwrap()
	    			}),
	    _ => None,
	}
}

fn parse_bot_rule(instruction: &str) -> Option<Bot> {
	let split_instr = instruction.split_whitespace().collect::<Vec<&str>>();
	match split_instr[0] {
	    "bot" => Some(Bot {
	    			bot_number: split_instr[1].to_string().parse().unwrap(),
	    			outputs: OutputRule {
	    				low: parse_output_rule(split_instr[5], split_instr[6]),
	    				high: parse_output_rule(split_instr[10], split_instr[11])
	    			} 	    				
	    		}),
	    _ => None,
	}
}

fn build_rules(instructions: &str) -> Rules {
	Rules {
		input_rules: instructions.lines().filter_map(|s| parse_input_rule(s)).collect(),
		bot_rules: instructions.lines().filter_map(|s| parse_bot_rule(s)).collect()
	}
}

fn main() {
	let final_state = run(include_str!("../input/input.txt"));
    println!("Bot Number (Part one) => {:?}", find_bot_number(&final_state.1, (61,17)));
    println!("Answer to part two => {:?}", final_state.0.bins[0] * final_state.0.bins[1] * final_state.0.bins[2]);
}

#[test]
fn instruction_test() {
	assert_eq!(Input {value: 5, to_bot: 2}, parse_input_rule("value 5 goes to bot 2").unwrap());
	assert_eq!(Bot {bot_number: 2, outputs: OutputRule{low: BotOutputType::Bot{bot_number: 1}, high: BotOutputType::Bot{bot_number: 0}}}, parse_bot_rule("bot 2 gives low to bot 1 and high to bot 0").unwrap());
}

#[test]
fn construction_line_test() {
	let input = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

	let test_rules = Rules {
		input_rules: vec!(
					Input {value: 5, to_bot: 2},
					Input {value: 3, to_bot: 1},
					Input {value: 2, to_bot: 2}
				),
		bot_rules: vec!(
				Bot {bot_number: 2, outputs: OutputRule{low: BotOutputType::Bot{bot_number: 1}, high: BotOutputType::Bot{bot_number: 0}}},
				Bot {bot_number: 1, outputs: OutputRule{low: BotOutputType::Bin{bin_number: 1}, high: BotOutputType::Bot{bot_number: 0}}},
				Bot {bot_number: 0, outputs: OutputRule{low: BotOutputType::Bin{bin_number: 2}, high: BotOutputType::Bin{bin_number: 0}}}
			)
	};

	assert_eq!(test_rules, build_rules(input));
}

#[test]
fn run_bots_test() {
	let input = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

	assert_eq!(State {bins: vec!(5, 2, 3), bots: vec!((-1,-1),(-1,-1),(-1,-1))}, run(input).0);
}

#[test]
fn bot_number_test() {
	let input = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

	assert_eq!(2, find_bot_number(&run(input).1, (5,2)));
}