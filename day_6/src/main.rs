fn char_to_index(c: char) -> usize {
    (c as u32 - 'a' as u32) as usize
}

fn index_to_char(i: usize) -> char {
    (i as u8 + 'a' as u8) as char
}

fn get_char_count_array(char_array: &Vec<char>) -> Vec<i32> {
    char_array.iter().fold(vec![0; 1 + ('z' as u32 - 'a' as u32) as usize], |count_array, c| {
            count_array.iter().enumerate().map(|(i,x)| if i == char_to_index(*c) {*x+1} else {*x}).collect::<Vec<i32>>()
    })
}

fn get_least_frequent_char(char_counts: &Vec<i32>) -> char {
    index_to_char(char_counts.iter().enumerate().fold((char_counts.len() as i32,0), |current_min, (i,count)| {
            if *count > 0 && *count < current_min.0 {
                (*count,i)
            }
            else {
                current_min
            }
    }).1)
}

fn get_most_frequent_char(char_counts: &Vec<i32>) -> char {
    index_to_char(char_counts.iter().enumerate().fold((0,0), |current_max, (i,count)| {
            if count > &current_max.0 {
                (*count,i)
            }
            else {
                current_max
            }
    }).1)
}

fn get_char_columns(message: &str) -> Vec<Vec<char>> {
    let lines = message.lines().collect::<Vec<&str>>();
    let num_rows = lines[0].len();
    let chars : Vec<char> = lines.iter().flat_map(|s| s.chars()).collect();

    (0..num_rows).fold(vec!(), |columns, row_index| {
        let next_column = vec!(chars.iter().enumerate().filter_map(|(i,c)| if i % num_rows == row_index {Some(*c)} else {None}).collect::<Vec<char>>());
        columns.iter().chain(next_column.iter()).cloned().collect()
    })
}

fn correct_message_part_one(message: &str) -> String {
    get_char_columns(message).iter().map(|x| {
        get_most_frequent_char(&get_char_count_array(x))
    }).collect()
}

fn correct_message_part_two(message: &str) -> String {
    get_char_columns(message).iter().map(|x| {
        get_least_frequent_char(&get_char_count_array(x))
    }).collect()
}

fn main() {
    println!("Corrected message (Part One) = {:?}", correct_message_part_one(include_str!("../input/input.txt")));
    println!("Corrected message (Part Two) = {:?}", correct_message_part_two(include_str!("../input/input.txt")));
}

#[test]
fn part_one() {
    let input = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    assert_eq!("easter", correct_message_part_one(&input));
}

#[test]
fn part_two() {
    let input = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    assert_eq!("advent", correct_message_part_two(&input));
} 