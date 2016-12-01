use std::io::prelude::*;
use std::fs::File;
use std::collections::{HashMap,HashSet};

fn main() {
   let mut file = File::open("input/input.txt").unwrap();
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();
   
   let (happiness, guests) = get_guests(&input);
   let happiness = calculate_happiness(guests, happiness);
   
   println!("{}", happiness);
}

fn calculate_happiness(guests: Vec<&str>, happy_map: HashMap<&str, HashMap<&str, i32>>) -> i32 {
    let mut table_layouts: Vec<i32> = get_permutations(guests).into_iter().map(|p| {
    	
    	//println!("---------------");

    	let mut happiness = 0;

    	for i in 0..p.len() {
    		let array_len: i32 = p.len() as i32;
    		let mut left_index: i32 = i as i32 - 1;
    		let mut right_index: i32 = i as i32 + 1;

    		if left_index < 0 {
    			left_index = array_len - 1;
    		}

    		if right_index == array_len {
    			right_index = 0;
    		}

    	    let left_index: usize = left_index as usize;
    	    let right_index: usize = right_index as usize;
    	    let ref guest_map = happy_map.get(p[i]).unwrap();
    	    let left_happy = *guest_map.get(p[left_index]).unwrap();
    	    let right_happy = *guest_map.get(p[right_index]).unwrap();

    	    //println!("{} {} {}", left_happy, p[i], right_happy);

    	    happiness += left_happy + right_happy;
    	}

    	//println!("{}", happiness);
    	//println!("---------------");

        happiness

    }).collect();

    table_layouts.sort();

    *table_layouts.last().unwrap()
}

fn get_permutations<T: Clone>(v: Vec<T>) -> Vec<Vec<T>> {
    match v.len() {
        0 | 1 => vec![v],
        2 => {
            let rev0 = v.get(1).unwrap().clone();
            let rev1 = v.get(0).unwrap().clone();
            vec![v, vec![rev0, rev1]]
        },
        _ => {
            let mut permutations = vec![];
            for i in 0..v.len() {
                let mut v2 = v.to_vec();
                v2.swap(0, i);
                let curr = v2.get(0).unwrap().clone();
                v2.remove(0);
                for mut p in get_permutations(v2.to_vec()) {
                    p.insert(0, curr.clone());
                    permutations.push(p);
                }
            }
            permutations
        },
    }
}

fn get_guests(input: &String) -> (HashMap<&str, HashMap<&str, i32>>, Vec<&str>) {
    let mut happiness: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    let mut guests: HashSet<&str> = HashSet::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split(|c| c == ' ' || c == '.').collect();
        let guest = split[0];
        let happiness_val = split[3].parse::<i32>().unwrap() * if split[2] == "gain" { 1 } else { -1 };
        let neigbour = split[10];
       	
       	//println!("{}: G={}, N={}, h={}", line, guest, neigbour, happiness_val);
        guests.insert(guest);
        guests.insert(neigbour);

        if !happiness.contains_key(guest) {
    	    let mut neighbour_map: HashMap<&str, i32> = HashMap::new();
    	    neighbour_map.insert("Jack", 0);
        	neighbour_map.insert(neigbour, happiness_val);
        	happiness.insert(guest, neighbour_map);
        } else {
            happiness.get_mut(guest).unwrap().insert(neigbour, happiness_val);
        }
    }

    let mut my_neighbour_map: HashMap<&str, i32> = HashMap::new();
    let mut g = vec![];
    for guest in guests {
    	my_neighbour_map.insert(guest, 0);
        g.push(guest);
    }

    g.push("Jack");
    happiness.insert("Jack", my_neighbour_map);

    (happiness, g)
}