use std::fmt;
use std::collections::HashMap;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
enum LocationType {
    Wall,
    Open,
}

impl fmt::Debug for LocationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &LocationType::Wall => "#",
            &LocationType::Open => "."
        })
    }
}

impl LocationType {
    fn construct(input: u32) -> LocationType {
        if input % 2 == 0 {
            LocationType::Open
        } else {
            LocationType::Wall
        }
    }
}

#[derive(Clone)]
struct Node {
    location: (usize, usize),
    distance: usize
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.location == other.location
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

fn get_location_type((x,y): (u64,u64), designer_num: u64) -> LocationType {
    LocationType::construct(((x*x + 3*x + 2*x*y + y + y*y) + designer_num).count_ones())
}

fn get_neighbours(node: (usize, usize), designer_num: u64) -> Vec<(usize, usize)> {
    (0..4).filter_map(|x| {
        let neighbour = match x {
            0 => if node.0 > 0 {Some((node.0 - 1, node.1))} else {None},
            1 => if node.1 > 0 {Some((node.0, node.1 - 1))} else {None},
            2 => Some((node.0 + 1, node.1)),
            3 => Some((node.0, node.1 + 1)),
            _ => None
        };

        if neighbour.is_some() && get_location_type((neighbour.unwrap().0 as u64, neighbour.unwrap().1 as u64), designer_num) == LocationType::Open {
            neighbour
        } else {
            None
        }
    }).collect()
}

fn get_path(came_from: HashMap<(usize, usize), (usize, usize)>, node: (usize, usize)) -> Vec<(usize, usize)> {
    let mut path = vec!(node);
    let mut current = node;
    while let Some(next_node) = came_from.get(&current) {
        current = *next_node;
        path.push(current);
    }

    path
}

fn find_reachable_nodes(from: (usize, usize), max_distance: usize, designer_num: u64) -> usize {
    let mut closed_set = Vec::new();
    let mut open_set = BinaryHeap::new();
    open_set.push(Node{location: from, distance: 0});

    let mut came_from = HashMap::new();
    let mut seen = HashMap::new();

    while let Some(node) = open_set.pop() {
        if node.distance <= max_distance {
            closed_set.push(node.location);

            for neighbour in get_neighbours(node.location, designer_num) {
                if !closed_set.contains(&neighbour) {
                    let dist = node.distance + 1;

                    {
                        let ref have_visited = seen.get(&neighbour);
                        if have_visited.is_none() {
                            open_set.push(Node{location: neighbour, distance: dist});
                        } else if *have_visited.unwrap() <= dist {
                            continue;
                        }
                    }

                    came_from.insert(neighbour, node.location);
                    seen.insert(neighbour, dist);
                }
            }
        }
    }

    closed_set.len()
}

fn find_shortest_route(from: (usize, usize), to: (usize, usize), designer_num: u64) -> Vec<(usize,usize)> {
    let mut closed_set = Vec::new();
    let mut open_set = BinaryHeap::new();
    open_set.push(Node{location: from, distance: 0});

    let mut came_from = HashMap::new();
    let mut seen = HashMap::new();

    while let Some(node) = open_set.pop() {
        if node.location == to {
            return get_path(came_from, node.location);
        }

        closed_set.push(node.location);

        for neighbour in get_neighbours(node.location, designer_num) {
            if !closed_set.contains(&neighbour) {
                let dist = node.distance + 1;

                {
                    let ref have_visited = seen.get(&neighbour);
                    if have_visited.is_none() {
                        open_set.push(Node{location: neighbour, distance: dist});
                    } else if *have_visited.unwrap() <= dist {
                        continue;
                    }
                }

                came_from.insert(neighbour, node.location);
                seen.insert(neighbour, dist);
            }
        }
    }

    panic!("Path not found");
}

fn draw_room((max_x,max_y): (usize, usize), designer_num: u64, path: &Vec<(usize, usize)>) {
    std::process::Command::new("clear").status().unwrap();
    for y in 0..max_y {
        for x in 0..max_x {
            if path.contains(&(x,y)) {
                print!("O");
            } else {
                print!("{:?}", get_location_type((x as u64, y as u64), designer_num));
            }      
        }
        println!("");
    }
}

fn main() {
    let path = find_shortest_route((1,1), (31,39), 1364);
    draw_room((50, 50), 1364, &path);
    println!("Num steps => {:?}", path.len() - 1);
    println!("Num nodes => {:?}", find_reachable_nodes((1,1), 50, 1364));
}

#[test]
fn test() {
    let path = find_shortest_route((1,1), (7,4), 10);
    assert_eq!(11, path.len()-1);
}