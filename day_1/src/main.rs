use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

#[derive(Debug)]
enum Facing {
    North,
    South,
    East,
    West
}

struct DirectionPair(Direction, i32);
struct Location(Facing, i32, i32);

fn main() {
    let directions: Vec<u8> = File::open("input/input.txt").unwrap().bytes().map(|b| b.unwrap()).collect();
    let directions_str : String = String::from_utf8_lossy(&directions).split_whitespace().collect();

    let Location(final_facing, final_x, final_y) : Location = directions_str.split(',').map(|d| {
        let (dir, dist) = d.split_at(1);
        DirectionPair(match dir {
            "R" => Direction::Right,
            "L" => Direction::Left,
            _   => panic!("Invalid direction letter: {}", dir)
        }, dist.parse().unwrap())
    }).fold(Location(Facing::North,0,0), |Location(facing, x, y), DirectionPair(dir, dist)| {
        let new_facing = match dir {
            Direction::Right => match facing {
               Facing::North => Facing::East,
               Facing::South => Facing::West,
               Facing::East => Facing::South,
               Facing::West => Facing::North
            },
            Direction::Left => match facing {
               Facing::North => Facing::West,
               Facing::South => Facing::East,
               Facing::East => Facing::North,
               Facing::West => Facing::South
            }
        };

        let (new_x, new_y) = match new_facing {
            Facing::North => (x, y + dist),
            Facing::South => (x, y - dist),
            Facing::East => (x + dist, y),
            Facing::West => (x - dist, y)
        };

        println!("Facing {:?}, X = {}, Y = {}", new_facing, new_x, new_y);

        Location(new_facing, new_x, new_y)
    });

    println!("Final Location {:?}, X = {}, Y = {}", final_facing, final_x, final_y);
    println!("Final Distance = {}", (final_x + final_y).abs());
}
