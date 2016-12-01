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
struct Path(Facing, Vec<(i32, i32)>);

fn main() {
    let directions: Vec<u8> = File::open("input/input.txt").unwrap().bytes().map(|b| b.unwrap()).collect();
    let directions_str : String = String::from_utf8_lossy(&directions).split_whitespace().collect();

    let Path(final_facing, path) : Path = directions_str.split(',').map(|d| {
        let (dir, dist) = d.split_at(1);
        DirectionPair(match dir {
            "R" => Direction::Right,
            "L" => Direction::Left,
            _   => panic!("Invalid direction letter: {}", dir)
        }, dist.parse().unwrap())
    }).fold(Path(Facing::North, vec![(0,0)]), | Path(facing, history), DirectionPair(dir, dist)| {
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

        let (old_x, old_y) = (history.last().unwrap().0, history.last().unwrap().1);
        let visited : Vec<(i32,i32)> = (0..dist).map(|x|{
            match new_facing {
            Facing::North => (old_x, old_y + (x+1)),
            Facing::South => (old_x, old_y - (x+1)),
            Facing::East => (old_x + (x+1), old_y),
            Facing::West => (old_x - (x+1), old_y)
            }
        }).collect();

        Path(new_facing, history.iter().chain(visited.iter()).cloned().collect())
    });

    let (final_x, final_y) = (path.last().unwrap().0, path.last().unwrap().1);
    println!("Final Location {:?}, X = {}, Y = {}", final_facing, final_x, final_y);
    println!("Final Distance = {}", (final_x.abs() + final_y.abs()).abs());

    let repeats : Vec<(&(i32,i32),i32)> = path.iter().enumerate().filter_map(|x| {
        let location : &(i32, i32) = x.1;
        match path.split_at((x.0)+1).1.contains(&location) {
            true => Some((location, location.0.abs() + location.1.abs())),
            false => None        
        }
    }).collect();

    let (first_repeat_location, first_repeat_distance) = (repeats.first().unwrap().0, repeats.first().unwrap().1);
    println!("First repeat = ({},{}), Distance = {}", first_repeat_location.0, first_repeat_location.1, first_repeat_distance);
}