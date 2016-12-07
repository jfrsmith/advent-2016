fn check_segment_for_ABBA(segment: &str) -> bool {
    
}

fn does_address_support_TLS(address_line: &str) -> bool {
    
}

fn get_TLS_address_count(address_lines: &str) -> i32 {
    address_lines.lines().fold(0, |acc, line| {
        acc + (does_address_support_TLS(line) as i32)
    })
}

fn main() {
   println!("Number of IPs that support TLS = {:?}", get_TLS_address_count(include_str!("../input/input.txt")));
   // println!("Corrected message (Part Two) = {:?}", correct_message_part_two(include_str!("../input/input.txt")));
}

#[test]
fn part_one() {
    assert_eq!(true, does_address_support_TLS("abba[mnop]qrst"));
    assert_eq!(false, does_address_support_TLS("abcd[bddb]xyyx"));
    assert_eq!(false, does_address_support_TLS("aaaa[qwer]tyui"));
    assert_eq!(true, does_address_support_TLS("ioxxoj[asdfgh]zxcvbn"));
}

#[test]
fn part_two() {
} 