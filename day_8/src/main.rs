use std::fmt;

#[derive(Debug,PartialEq)]
enum Instruction {
    Rect{x: i32, y: i32},
    RotateRow{row_index: i32, rotate_by: i32},
    RotateCol{col_index: i32, rotate_by: i32}
}

struct Screen {
    grid: Vec<Vec<bool>>
}

impl Screen {
    fn get_lit_pixels(&self) -> i32 {
        self.grid.iter().fold(0, |acc_col, row| {
            row.iter().fold(acc_col, | acc_row, pixel | {
                acc_row + (*pixel as i32)
            })
        })
    }

    fn render_screen(&self) -> String {
        self.grid.iter().enumerate().fold(vec!(), | grid_output, (row_index, row) | {
            let row = row.iter().fold(vec!(), | row_output, cell | {
                let cell_content = match *cell {
                    true => '#',
                    false => '.'
                };
                row_output.iter().chain(vec!(cell_content).iter()).cloned().collect()
            });

            let is_last_line = row_index == (self.grid.len() - 1);

            let row_finalised = match is_last_line {
                true => row,
                false => row.iter().chain(vec!('\n').iter()).cloned().collect::<Vec<char>>()
            };

            grid_output.iter().chain(row_finalised.iter()).cloned().collect()
        }).iter().map(|c| *c).collect()
    }
}

impl fmt::Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.render_screen())
    }
}

fn modulo(x:i32, d:i32) -> i32 {
    if x < 0 {
        (x % d) + d
    }
    else {
        x % d
    }
}

fn parse_rect_instruction(params: &[&str]) -> Instruction {
    let dims = params[0].split('x').collect::<Vec<&str>>();
    Instruction::Rect{x: dims.get(0).unwrap().parse().unwrap(), y: dims.get(1).unwrap().parse().unwrap() }
}

fn parse_rotation_instruction(params: &[&str]) -> Instruction {
    let rotate_params = (params[1].split('=').last().unwrap().parse().unwrap(), params[3].parse().unwrap());

    match params[0] {
        "column" => Instruction::RotateCol{col_index: rotate_params.0, rotate_by: rotate_params.1},
        "row" => Instruction::RotateRow{row_index: rotate_params.0, rotate_by: rotate_params.1},
        x =>  panic!("Unexpected rotation parameter {:?}", x)
    }
}

fn parse_instruction(instruction: &str) -> Instruction {
    let split_instr = instruction.split_whitespace().collect::<Vec<&str>>();
    match split_instr[0] {
        "rect" => parse_rect_instruction(split_instr.split_at(1).1),
        "rotate" => parse_rotation_instruction(split_instr.split_at(1).1),
        x => panic!("Unexpected instruction {:?}", x)
    }
}

fn construct_screen(screen_dimensions: (usize, usize)) -> Screen {
    Screen { grid: vec!(vec!(false; screen_dimensions.0); screen_dimensions.1)}
}

fn apply_instruction(next_instruction: Instruction, last_state: &Screen) -> Screen {
    let new_grid = last_state.grid.iter().enumerate().map(|(row_index, row)| {
        row.iter().enumerate().map(|(col_index, _)| {
            match next_instruction {
                Instruction::Rect{x, y} => {
                    (col_index < x as usize && row_index < y as usize) || last_state.grid[row_index][col_index]
                },
                Instruction::RotateRow{row_index: x, rotate_by: r} => {
                    if row_index == x as usize {
                        let prev_col_index = modulo((col_index as i32 - r), last_state.grid[0].len() as i32) as usize;
                        last_state.grid[row_index][prev_col_index]
                    }
                    else {
                        last_state.grid[row_index][col_index]
                    }
                },
                Instruction::RotateCol{col_index: y, rotate_by: r} => {
                    if col_index == y as usize {
                        let prev_row_index = modulo((row_index as i32 - r), last_state.grid.len() as i32) as usize;
                        last_state.grid[prev_row_index][col_index]
                    }
                    else {
                        last_state.grid[row_index][col_index]
                    }
                }
            }
        }).collect::<Vec<bool>>()
    }).collect::<Vec<Vec<bool>>>();

    Screen {grid: new_grid.clone()}
}

fn count_lit_pixels(instructions: &str, screen_dimensions: (usize, usize)) -> i32 {
    let initial_screen = construct_screen(screen_dimensions);
    instructions.lines().map(|instruction_line| {parse_instruction(instruction_line)}).fold(initial_screen, | last_state, instr | {
        let new_grid = apply_instruction(instr, &last_state);

        println!("{:?}\n", new_grid);
        new_grid
    }).get_lit_pixels()
}

fn main() {
    println!("Lit pixels => {:?}", count_lit_pixels(include_str!("../input/input.txt"), (50,6)));
}

#[test]
fn instruction_parsing() {
    assert_eq!(Instruction::Rect{x: 3, y: 2}, parse_instruction("rect 3x2"));
    assert_eq!(Instruction::RotateCol{col_index: 1, rotate_by: 1}, parse_instruction("rotate column x=1 by 1"));
    assert_eq!(Instruction::RotateRow{row_index: 0, rotate_by: 4}, parse_instruction("rotate row y=0 by 4"));
}

#[test]
fn instruction_sequence() {
    let input = "rect 3x2
    rotate column x=1 by 1
    rotate row y=0 by 4
    rotate column x=1 by 1";
    assert_eq!(6, count_lit_pixels(input, (7,3)));
}

#[test]
fn test_instructions() {
    let mut input = "rect 3x2";
    let mut output = "###....
###....
.......";

    let mut last_state = apply_instruction(parse_instruction(&input), &construct_screen((7,3)));
    assert_eq!(output, last_state.render_screen());

    input = "rotate column x=1 by 1";
    output = "#.#....
###....
.#.....";

    last_state = apply_instruction(parse_instruction(&input), &last_state);
    assert_eq!(output, last_state.render_screen());

    input = "rotate row y=0 by 4";
    output = "....#.#
###....
.#.....";

    last_state = apply_instruction(parse_instruction(&input), &last_state);
    assert_eq!(output, last_state.render_screen());

    input = "rotate column x=1 by 1";
    output = ".#..#.#
#.#....
.#.....";

    last_state = apply_instruction(parse_instruction(&input), &last_state);
    assert_eq!(output, last_state.render_screen());
}

#[test]
fn output_test() {
    let screen_test = ".......
.......
.......";

    assert_eq!(screen_test, construct_screen((7,3)).render_screen());
}