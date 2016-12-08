fn check_segment_for_abba(segment: &str) -> bool {
    for window in segment.chars().collect::<Vec<char>>().windows(4) {
        if window[0] != window[1] && window[0] == window[3] && window[1] == window[2] {
            return true;
        }
    }

    false
}

fn check_segment_for_aba(segment: &str) -> Vec<String> {
    segment.chars().collect::<Vec<char>>().windows(3).fold(vec!(), | aba_vec, window | {
        if window[0] != window[1] && window[0] == window[2] {
            return aba_vec.iter().chain(vec![window.into_iter().map(|c| *c).collect()].iter()).cloned().collect();
        }

        aba_vec
    })
}

fn reverse_aba(aba: &str) -> String {
    vec!(aba.chars().nth(1).unwrap(), aba.chars().nth(0).unwrap(), aba.chars().nth(1).unwrap()).into_iter().collect()
}

fn does_address_support_ssl(address_line: &str) -> bool {
    let (supernet_aba, hypernet_aba) = address_line.split(|c| c == '[' || c == ']').enumerate().fold((vec!(), vec!()), | (supernet, hypernet), (segment_index, segment) | {
        let abba_segment = check_segment_for_aba(segment);

        if !abba_segment.is_empty() {
            if segment_index % 2 == 0 {
                return (supernet.iter().chain(abba_segment.iter()).cloned().collect(), hypernet);
            }
            else {
                return (supernet, hypernet.iter().chain(abba_segment.iter()).cloned().collect());
            }
        }

        (supernet, hypernet)
    });

    for aba in supernet_aba {
        if hypernet_aba.contains(&reverse_aba(&aba)) {
            return true;
        }
    }

    false
}

fn does_address_support_tls(address_line: &str) -> bool {
    let counts = address_line.split(|c| c == '[' || c == ']').enumerate().fold((0, 0), | (matched_segments, matched_hypernets), (segment_index, segment) | {
        let abba_segment = check_segment_for_abba(segment);

        if segment_index % 2 != 0 && abba_segment {
            return (matched_segments, matched_hypernets + 1);
        }

        (matched_segments + (abba_segment as i32), matched_hypernets)
    });

    counts.0 > 0 && counts.1 == 0
}

fn get_tls_address_count(address_lines: &str) -> i32 {
    address_lines.lines().fold(0, |acc, line| {
        acc + (does_address_support_tls(line) as i32)
    })
}

fn get_ssl_address_count(address_lines: &str) -> i32 {
    address_lines.lines().fold(0, |acc, line| {
        acc + (does_address_support_ssl(line) as i32)
    })
}

fn main() {
   println!("Number of IPs that support TLS = {:?}", get_tls_address_count(include_str!("../input/input.txt")));
   println!("Number of IPs that support SSL = {:?}", get_ssl_address_count(include_str!("../input/input.txt")));
}

#[test]
fn part_one() {
    assert_eq!(true, does_address_support_tls("abba[mnop]qrst"));
    assert_eq!(false, does_address_support_tls("abcd[bddb]xyyx"));
    assert_eq!(false, does_address_support_tls("aaaa[qwer]tyui"));
    assert_eq!(true, does_address_support_tls("ioxxoj[asdfgh]zxcvbn"));
}

#[test]
fn part_two() {
    assert_eq!(true, does_address_support_ssl("aba[bab]xyz"));
    assert_eq!(false, does_address_support_ssl("xyx[xyx]xyx"));
    assert_eq!(true, does_address_support_ssl("aaa[kek]eke"));
    assert_eq!(true, does_address_support_ssl("zazbz[bzb]cdb"));
} 