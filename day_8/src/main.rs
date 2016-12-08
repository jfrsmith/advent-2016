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
        row.iter().enumerate().map(|(col_index, pixel)| {
            match next_instruction {
                //If this co-ordinate is within the bounds of the rect, then we return true as it should be lit
                Instruction::Rect{x: x, y: y} => {
                    row_index < x as usize && col_index < y as usize
                },
                //If, in the previous state, the reverse of these instructions indicate a lit pixel, then this should be lit
                Instruction::RotateRow{row_index: x, rotate_by: r} => {
                    
                },
                Instruction::RotateCol{col_index: y, rotate_by: r} => {

                }
            }
        }).collect::<Vec<bool>>()
    }).collect::<Vec<Vec<bool>>>();

    Screen {grid: new_grid.clone()}
}

fn count_lit_pixels(instructions: &str, screen_dimensions: (usize, usize)) -> i32 {
    let initial_screen = construct_screen(screen_dimensions);
    instructions.lines().map(|instruction_line| {parse_instruction(instruction_line)}).fold(initial_screen, | last_state, instr | {
        apply_instruction(instr, &last_state)
    }).get_lit_pixels()
}

fn main() {
    print!("Lit pixels => {:?}", count_lit_pixels(include_str!("../input/input.txt"), (50,6)));
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
    rotate row y=0 by 4";
    assert_eq!(6, count_lit_pixels(input, (7,3)));
}

#[test]
fn output_test() {
    let screen_test = ".......
.......
.......";

    assert_eq!(screen_test, construct_screen((7,3)).render_screen());
}