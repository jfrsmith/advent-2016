use std::fmt;

#[derive(Debug, Eq, PartialEq)]
enum Val {
    Raw{val: i32},
    Reg{ptr: usize}
}

#[derive(Eq, PartialEq)]
enum Instruction {
    Copy{val: Val, ptr: usize},
    Inc{ptr:usize},
    Dec{ptr:usize},
    Jump{val: Val, dist: i32}
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl Instruction {
    fn to_str(&self) -> String {
        match self {
            &Instruction::Copy{ref val, ptr} => {
                format!("cpy {} {}", match val {
                    &Val::Raw{val: inner_val} => inner_val.to_string(),
                    &Val::Reg{ptr: inner_ptr} => from_reg_index(inner_ptr),
                }, from_reg_index(ptr))
            },
            &Instruction::Inc{ptr} => {
                format!("inc {}", from_reg_index(ptr))
            },
            &Instruction::Dec{ptr} => {
                format!("dec {}", from_reg_index(ptr))
            },
            &Instruction::Jump{ref val, dist} => {
                format!("jnz {} {}", match val {
                    &Val::Raw{val: inner_val} => inner_val.to_string(),
                    &Val::Reg{ptr: inner_ptr} => from_reg_index(inner_ptr),
                }, dist)
            }
        } 
    }

    fn run(&self, registers: &Vec<i32>) -> (Vec<i32>, i32) {
        match self {
            &Instruction::Copy{ref val, ptr} => {
                let copy_val = match val {
                    &Val::Raw{val: inner_val} => inner_val,
                    &Val::Reg{ptr: inner_ptr} => registers[inner_ptr],
                };

                (registers.iter().enumerate().map(|(i, x)| if i == ptr {copy_val} else {*x}).collect(), 1)
            },
            &Instruction::Inc{ptr} => {
                (registers.iter().enumerate().map(|(i, x)| if i == ptr {x + 1} else {*x}).collect(), 1)
            },
            &Instruction::Dec{ptr} => {
                (registers.iter().enumerate().map(|(i, x)| if i == ptr {x - 1} else {*x}).collect(), 1)
            },
            &Instruction::Jump{ref val, dist} => {
                let should_jump = match val {
                    &Val::Raw{val: inner_val} => inner_val != 0,
                    &Val::Reg{ptr: inner_ptr} => registers[inner_ptr] != 0,
                };

                (registers.to_vec(), if should_jump {dist} else {1})
            }
        }
    }
}

fn from_reg_index(i: usize) -> String {
    match i {
        0 => "a".to_string(),
        1 => "b".to_string(),
        2 => "c".to_string(),
        3 => "d".to_string(),
        _ => panic!("Unknown reg index {}", i)
    }
}

fn to_reg_index(reg: &str) -> usize {
    match reg {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        x   => panic!("Unknown register {}", x)
    }
}

fn parse_val(val_str: &str) -> Val {
    match val_str.parse::<i32>() {
        Ok(x) => Val::Raw{val: x},
        Err(_) => Val::Reg{ptr: to_reg_index(val_str)}
    }
}

fn parse_instruction(split_instr: Vec<&str>) -> Instruction {
    match split_instr[0] {
        "cpy" => Instruction::Copy {
            val: parse_val(split_instr[1]),
            ptr: to_reg_index(split_instr[2])
        },
        "inc" => Instruction::Inc {
            ptr: to_reg_index(split_instr[1])
        },
        "dec" => Instruction::Dec {
            ptr: to_reg_index(split_instr[1])
        },
        "jnz" => Instruction::Jump {
            val: parse_val(split_instr[1]),
            dist: split_instr[2].parse::<i32>().unwrap()
        },
        x => panic!("Unknown instruction {:?}", x),
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| {
        parse_instruction(line.split_whitespace().collect())
    }).collect()
}

fn print(instructions: &Vec<Instruction>, ptr: usize, registers: &Vec<i32>) {
    std::process::Command::new("clear").status().unwrap();

    for (i, instr) in instructions.iter().enumerate() {
        println!("{} {:?}", if i == ptr {"->"} else {"  "}, instr);
    }

    println!("");

    for (i, reg) in registers.iter().enumerate() {
        println!("{} {}", from_reg_index(i), reg);
    }
}

fn run(instructions: &Vec<Instruction>) -> Vec<i32> {
    let mut instruction_pointer = 0;
    let mut registers = vec!(0; 4);
    let instruction_stack = instructions.clone();

    while instruction_pointer < instruction_stack.len() {
        let (new_registers, next_instr) = instruction_stack[instruction_pointer].run(&registers);
        print(&instruction_stack, instruction_pointer, &new_registers);
        instruction_pointer = (instruction_pointer as i32 + next_instr) as usize;
        registers = new_registers;
    }

    registers.to_vec()
}

fn main() {
    println!("Value in register a = {}", run(&parse_instructions(include_str!("../input/input.txt")))[to_reg_index("a")]);
}

#[test]
fn parse() {
    let input = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

    let expected = vec!(Instruction::Copy{val: Val::Raw{val: 41}, ptr: 0},
                        Instruction::Inc{ptr: 0},
                        Instruction::Inc{ptr: 0},
                        Instruction::Dec{ptr: 0},
                        Instruction::Jump{val: Val::Reg{ptr: 0}, dist: 2},
                        Instruction::Dec{ptr: 0});

    let output = parse_instructions(&input);

    println!("{:#?}", expected);
    println!("{:#?}", output);

    assert_eq!(expected, output);
}

#[test]
fn run_test() {
    let input = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

    assert_eq!(42, run(&parse_instructions(&input))[to_reg_index("a")]);
}

#[test]
fn cpy() {
    let input = "cpy 41 a";
    let registers = vec!(0; 4);

    let ref instr = parse_instructions(&input)[0];

    assert_eq!((vec!(41,0,0,0), 1), instr.run(&registers));
}

#[test]
fn inc() {
    let input = "inc a";
    let registers = vec!(41,0,0,0);

    let ref instr = parse_instructions(&input)[0];

    assert_eq!((vec!(42,0,0,0), 1), instr.run(&registers));
}

#[test]
fn dec() {
    let input = "dec a";
    let registers = vec!(42,0,0,0);

    let ref instr = parse_instructions(&input)[0];

    assert_eq!((vec!(41,0,0,0), 1), instr.run(&registers));
}

#[test]
fn jnz() {
    let input = "jnz a 2";
    let registers = vec!(41,0,0,0);

    let ref instr = parse_instructions(&input)[0];

    assert_eq!((vec!(41,0,0,0), 2), instr.run(&registers));
}