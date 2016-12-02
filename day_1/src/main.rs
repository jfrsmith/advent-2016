enum Direction {
    Left,
    Right
}

enum Facing {
    North,
    South,
    East,
    West
}

struct DirectionPair(Direction, i32);
struct Path(Facing, Vec<(i32, i32)>);

fn get_path(instructions: &str) -> Path {
    instructions.split(", ").map(|d| {
        let (dir, dist) = d.split_at(1);
        DirectionPair(match dir {
            "R" => Direction::Right,
            "L" => Direction::Left,
            _   => panic!("Invalid direction letter: {}", dir)
        }, dist.parse().unwrap())
    })
    .fold(Path(Facing::North, vec![(0,0)]), | Path(facing, history), DirectionPair(dir, dist)| {
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
    })
}

fn calculate_final_distance(path_taken: &Path) -> ((i32,i32),i32) {
    let &(final_x, final_y) = path_taken.1.last().unwrap();
    ((final_x, final_y), (final_x.abs() + final_y.abs()).abs())
}

fn calculate_first_intersect(path_taken: &Path) -> ((i32,i32),i32) {
    let first_intersect : &(i32,i32) = path_taken.1.iter().enumerate().filter_map(|x| {
        match path_taken.1.split_at((x.0)+1).1.contains(x.1) {
            true => Some(x.1),
            false => None        
        }
    }).collect::<Vec<&(i32,i32)>>().first().unwrap();
    ((first_intersect.0, first_intersect.1), (first_intersect.0.abs() + first_intersect.1.abs()).abs())
}

fn main() {
    let full_path = get_path(include_str!("../input/input.txt"));

    println!("Final Distance: {}", calculate_final_distance(&full_path).1);
    println!("First Intersect Distance: {}", calculate_first_intersect(&full_path).1);
}

#[test]
fn part_one() {
    let inputs = "R5, L5, R5, R3";
    assert_eq!(((10,2), 12), calculate_final_distance(&get_path(inputs)));
    let inputs = "R2, R2, R2";
    assert_eq!(((0,-2), 2), calculate_final_distance(&get_path(inputs)));
    let inputs = "R2, L3";
    assert_eq!(((2,3), 5), calculate_final_distance(&get_path(inputs)));
}

#[test]
fn part_two() {
    let inputs = "R8, R4, R4, R8";
    assert_eq!(((4,0), 4), calculate_first_intersect(&get_path(inputs)));
}