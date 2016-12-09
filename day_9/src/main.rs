fn decompress(input: &str, decompress_repeated_segments: bool) -> i64 {
    let next_marker_start = input.find('(');
    let next_marker_end = match next_marker_start {
        Some(x) => input[x..input.len()].find(')'),
        None => None
    };

    if next_marker_start.is_none() || next_marker_end.is_none() {
        return input.len() as i64;
    }

    let marker_start = next_marker_start.unwrap();
    let marker_end = marker_start + next_marker_end.unwrap();
    let decode_marker = input[marker_start+1..marker_end].split(|c| c == '(' || c == ')' || c == 'x').collect::<Vec<&str>>();
    let sequence_start = marker_end + 1;
    let sequence_end = sequence_start+decode_marker[0].parse::<usize>().unwrap();

    if decompress_repeated_segments {
        next_marker_start.unwrap() as i64 + (decompress(&input[sequence_start..sequence_end], true) * decode_marker[1].parse::<i64>().unwrap()) + decompress(&input[sequence_end..input.len()], true)
    }
    else {
        next_marker_start.unwrap() as i64 + ((sequence_end - sequence_start) as i64 * decode_marker[1].parse::<i64>().unwrap()) + decompress(&input[sequence_end..input.len()], false)
    }
}

fn main() {
    println!("Decompressed file length (Part one) => {:?}", decompress(include_str!("../input/input.txt"), false));
    println!("Decompressed file length (Part two) => {:?}", decompress(include_str!("../input/input.txt"), true));
}

#[test]
fn decompression_part_one() {
    let mut input = "ADVENT";
    assert_eq!(6, decompress(&input, false));

    input = "A(1x5)BC";
    assert_eq!(7, decompress(&input, false));

    input = "(3x3)XYZ";
    assert_eq!(9, decompress(&input, false));
    
    input = "A(2x2)BCD(2x2)EFG";
    assert_eq!(11, decompress(&input, false));
    
    input = "(6x1)(1x3)A";
    assert_eq!(6, decompress(&input, false));
    
    input = "X(8x2)(3x3)ABCY";
    assert_eq!(18, decompress(&input, false));
}

#[test]
fn decompression_part_2() {
    let mut input = "(3x3)XYZ";
    assert_eq!(9, decompress(&input, true));

    input = "X(8x2)(3x3)ABCY";
    assert_eq!(20, decompress(&input, true));

    input = "(27x12)(20x12)(13x14)(7x10)(1x12)A";
    assert_eq!(241920, decompress(&input, true));
    
    input = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";
    assert_eq!(445, decompress(&input, true));
}