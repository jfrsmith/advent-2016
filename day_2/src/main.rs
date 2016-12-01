use std::io::{BufReader,BufRead};
use std::fs::File;
use std::cmp;

fn get_wrapping_paper_size(input_str: &str) -> (i32, i32) {
	let dimensions: Vec<_> = input_str.split("x").collect();
	let dim_as_num: Vec<i32> = dimensions.into_iter().map(|dim| dim.parse().unwrap()).collect();
	
	let w = dim_as_num[0];
	let l = dim_as_num[1];
	let h = dim_as_num[2];

	let a = w * l;
	let b = l * h;
	let c = h * w;

	let paper = (2 * a) + (2 * b) + (2 * c) + cmp::min(cmp::min(a, b), c);
	let ribbon = ((2 * w) + (2 * l) + (2 * h)  - (2 * cmp::max(cmp::max(w, l), h))) + (w * l * h);

	(paper, ribbon)
}

fn main() {
	let mut total_paper = 0;
	let mut total_ribbon = 0;
    let file = File::open("input/input.txt").unwrap();
    for line in BufReader::new(file).lines() {
    	let (paper, ribbon) = get_wrapping_paper_size(&line.unwrap());
    	total_paper += paper;
    	total_ribbon += ribbon;
    }

    println!("total_paper: {} total_ribbon: {}", total_paper, total_ribbon);
}