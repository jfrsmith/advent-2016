use std::io::prelude::*;
use std::fs::File;
use std::cmp;

#[derive(Debug)]
struct Reindeer<'a> {
    name: &'a str,
    speed: i32,
    fly_time: i32,
    rest_time: i32,
    distance: i32,
    score: i32,
    is_flying: bool,
    time_in_state: i32
}

fn main() {
   let mut file = File::open("input/input.txt").unwrap();
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();
   
   let mut reindeer = get_reindeer(&input);

   println!("race1 = {}", race_reindeer(&reindeer, 2503));
   println!("race2 = {}", race_reindeer_v2(&mut reindeer, 2503));
}

fn race_reindeer_v2(reindeer: &mut Vec<Reindeer>, duration: i32) -> i32 {
    let mut time_remaining = duration;
    let mut last_leader = -1;
    while time_remaining > 0 {

    	let mut max_distance = 0;
        
        for i in 0..reindeer.len() {
        	let ref mut racer = reindeer[i];

        	let mut max_time_in_state = 0;
            if racer.is_flying {
                racer.distance += racer.speed;
                max_time_in_state = racer.fly_time;
            }
            else {
                max_time_in_state = racer.rest_time;
            }

            if racer.distance > max_distance {
                max_distance = racer.distance;
            }

            racer.time_in_state += 1;
            if racer.time_in_state >= max_time_in_state {
                racer.time_in_state = 0;
                racer.is_flying = !racer.is_flying;
            }
        }

        for i in 0..reindeer.len() {
            if reindeer[i].distance == max_distance {
                reindeer[i].score += 1;
            }
        }

        time_remaining -= 1;
	}

    let mut max_score = 0;
    for racer in reindeer {
        if racer.score > max_score {
            max_score = racer.score;
        }
    }

    max_score
}

fn race_reindeer(reindeer: &Vec<Reindeer>, duration: i32) -> i32 {
    let mut distances: Vec<i32> = reindeer.into_iter().map(|p| {
    	let mut dist = 0;
    	let mut time_remaining = duration;
        let mut is_flying = true;

    	while time_remaining > 0 {
    		let loop_duration = cmp::min(time_remaining, if is_flying { p.fly_time } else { p.rest_time });
    		if is_flying {
    		    dist += loop_duration * p.speed;
    		}
    		time_remaining -= loop_duration;
    		is_flying = !is_flying;
    	}

        dist
    }).collect();

    distances.sort();

    *distances.last().unwrap()
}

fn get_reindeer(input: &String) -> Vec<Reindeer> {
    let mut reindeer: Vec<Reindeer> = Vec::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let name = split[0];
        let speed = split[3].parse::<i32>().unwrap();
        let fly_time = split[6].parse::<i32>().unwrap();
       	let rest_time = split[13].parse::<i32>().unwrap();

       	println!("{}: R={}, s={}, f={} r={}", line, name, speed, fly_time, rest_time);
        reindeer.push(Reindeer{name: name, speed: speed, fly_time: fly_time, rest_time: rest_time, 
        						is_flying: true, time_in_state: 0, distance: 0, score: 0});
    }

    reindeer
}