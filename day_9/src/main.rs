use std::io::prelude::*;
use std::fs::File;
use std::collections::{HashMap,HashSet};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Route<'a> {
    origin: &'a str,
    dest: &'a str
}

fn main() {
   let mut file = File::open("input/input.txt").unwrap();
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();
   
   let (routes, cities) = build_map(&input);
   let range = route_len_range(routes, cities);
   
   println!("{:?}", range);
}

fn route_len_range(routes: HashMap<Route, u32>, cities: Vec<&str>) -> (u32, u32) {
    let mut distances: Vec<u32> = get_permutations(cities).into_iter().map(|p| {
    	let mut dist = 0;
        for i in 0..p.len()-1 {
            let c1 = p.get(i).unwrap();
            let c2 = p.get(i+1).unwrap();
            let r = Route{origin: c1, dest: c2};
            dist += *routes.get(&r).unwrap();
        }
        dist
    }).collect();

    distances.sort();

    (*distances.first().unwrap(), *distances.last().unwrap())
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

fn build_map(input: &String) -> (HashMap<Route, u32>, Vec<&str>) {
    let mut routes: HashMap<Route, u32> = HashMap::new();
    let mut cities: HashSet<&str> = HashSet::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let origin = split[0];
        let dest = split[2];
        let dist = split[4].parse::<u32>().unwrap();
        //println!("{}: O={}, D={}, d={}", line, origin, dest, dist);
        routes.insert(Route{origin: origin, dest: dest}, dist);
        routes.insert(Route{origin: dest, dest: origin}, dist);
        cities.insert(origin);
        cities.insert(dest);
    }
    let mut cs = vec![];
    for city in cities {
        cs.push(city);
    }
    (routes, cs)
}