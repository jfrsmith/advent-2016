struct Triangle(i32, i32, i32);

fn is_valid_triangle(triangle: &Triangle) -> bool {
    ((triangle.0 + triangle.1) > triangle.2) && 
    ((triangle.1 + triangle.2) > triangle.0) &&
    ((triangle.2 + triangle.0) > triangle.1) 
}

fn get_num_triangles_by_row(input_str: &str) -> usize {
   input_str.lines().filter_map(|s| {
        match is_valid_triangle(&(s.split_whitespace()
            .enumerate().fold(Triangle(0,0,0), | Triangle(a,b,c), (i,s) | {
            match i {
                0 => Triangle(s.parse().unwrap(), b, c),
                1 => Triangle(a, s.parse().unwrap(), c),
                2 => Triangle(a, b, s.parse().unwrap()),
                _ => panic!("Invalid triangle size")
            }
        }))) {
            true => Some(s),
            false => None
        }
   }).collect::<Vec<&str>>().len()
}

fn get_num_triangles_by_col(input_str: &str) -> usize {
    input_str.lines().collect::<Vec<&str>>().chunks(3).map( |s| {
        s[0].split_whitespace().zip(s[1].split_whitespace()).zip(s[2].split_whitespace()).map( | ((x,y),z) | {
            Triangle(x.parse().unwrap(),y.parse().unwrap(),z.parse().unwrap())
        }).collect::<Vec<(Triangle)>>()
    }).fold(0, |outer_count, outer| {
        outer.iter().fold(outer_count, | inner_count, inner | {
            inner_count + (is_valid_triangle(inner) as usize)
        })
    })
}

fn main() {
    println!("Number of triangles (rows): {}", get_num_triangles_by_row(include_str!("../input/input.txt")));
    println!("Number of triangles (cols): {}", get_num_triangles_by_col(include_str!("../input/input.txt")));
}

#[test]
fn part_one() {
    let inputs = "5 10 25";
    assert_eq!(0, get_num_triangles_by_row(&inputs));
}

#[test]
fn part_two() {
    let inputs = "101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603";
    assert_eq!(6, get_num_triangles_by_col(&inputs));
}