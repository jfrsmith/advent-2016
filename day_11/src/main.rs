use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt;
use std::thread;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum SearchDir {
    Upwards,
    Downwards,
}

#[derive(Clone)]
struct Node {
    state: ContainmentAreaState,
    distance: usize,
    score: i32,
    search_dir: SearchDir
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> std::cmp::Ordering {
        match self.search_dir {
            SearchDir::Upwards => self.score.cmp(&other.score),
            SearchDir::Downwards => other.score.cmp(&self.score)
        }
    }
}

#[derive(Clone, Hash)]
struct ContainmentAreaState {
    top_floor: usize,
    elevator: usize,
    components: Vec<usize>,
    elements: Vec<String>,
    search_dir: SearchDir
}

impl Eq for ContainmentAreaState {}

impl PartialEq for ContainmentAreaState {
    fn eq(&self, other: &ContainmentAreaState) -> bool {
        self.components == other.components
    }
}

impl fmt::Debug for ContainmentAreaState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.output())
    }
}

impl ContainmentAreaState {
    fn get_score(&self) -> i32 {
        self.components.iter().fold(0, |score, x| score + (*x as i32 * *x as i32))
    }

    fn parse(input: &str) -> ContainmentAreaState {
        let (generators, microchips) : (Vec<(String, usize)>, Vec<(String,usize)>) = input.lines().enumerate().fold((vec!(), vec!()), |(g_vec, m_vec), (floor_index, line)| {
            if line.contains("nothing relevant") {
                return (g_vec, m_vec);
            }

            let floor_generators = line.rmatch_indices("generator").fold(vec!(), |generators, (i, _)| {
                generators.iter().chain(vec!((line[0..i].split_whitespace().last().unwrap().to_string(), floor_index)).iter()).cloned().collect()
            });

            let floor_microchips = line.rmatch_indices("microchip").fold(vec!(), |microchips, (i, _)| {
                microchips.iter().chain(vec!((line[0..i].split_whitespace().last().unwrap().split('-').nth(0).unwrap().to_string(), floor_index)).iter()).cloned().collect()
            });

            (
                g_vec.iter().chain(floor_generators.iter()).cloned().collect(),
                m_vec.iter().chain(floor_microchips.iter()).cloned().collect()
            )
        });

        let (components, elements) : (Vec<usize>, Vec<String>) = microchips.iter().fold((vec!(), vec!()), |(comp_vec, elem_vec), &(ref microchip_element_name, microchip_floor)| {
            let next_two_components = vec!(microchip_floor, generators.iter().find(|&&(ref gen_name, _)| gen_name == microchip_element_name).unwrap().1);

            (
                comp_vec.iter().chain(next_two_components.iter()).cloned().collect(),
                elem_vec.iter().chain(vec!(microchip_element_name.to_string()).iter()).cloned().collect()
            )
        });

        ContainmentAreaState {
            elevator: 0,
            components: components.clone(),
            elements: elements.clone(),
            top_floor: input.lines().clone().count(),
            search_dir: SearchDir::Upwards
        }
    }

    fn is_microchip(&self, component_index: usize) -> bool {
        component_index % 2 == 0
    }

    fn get_component_name(&self, component_index: usize) -> String {
        let is_microchip = self.is_microchip(component_index);
        let element_index = if is_microchip { component_index / 2 } else {(component_index - 1)/2};
        let element_name = self.elements[element_index][0..2].to_uppercase().to_string();
        let type_name = if is_microchip {"m"} else {"g"};

        format!("{}{}", element_name, type_name).to_string()
    }

    fn output(&self) -> String {
        (0..self.top_floor).rev().fold("".to_string(), |out_str, floor_index| {
            let components_str = self.components.iter().enumerate().map(|(i, comp)| {
                if *comp as usize == floor_index {
                    format!("{}", self.get_component_name(i))
                } else {
                    " . ".to_string()
                }
            }).collect::<Vec<String>>().join("  ");

            out_str + &format!("F{} {} {}\n", floor_index+1, if self.elevator == floor_index {"E  "} else {".  "}, components_str)
        })
    }

    fn is_floor_empty(&self, floor: usize) -> bool {
        self.components.iter().all(|&f| f != floor)
    }

    fn state_is_valid(&self) -> bool {
        self.components.iter().enumerate().all(|(i,&floor)| {
             (
                //I'm not a microchip
                !self.is_microchip(i) ||
                //I'm a microchip and i'm on the same floor as my generator
                self.components[i+1] == floor ||
                //I'm a microchip and the rest of my floor is empty or microchips
                self.components.iter().enumerate().all(|(j,&inner_floor)| (i == j || floor != inner_floor) || self.is_microchip(j))
            )
        })
    }

    fn permute_components(&self, component_indices: Vec<usize>, direction: i32) -> ContainmentAreaState {
        ContainmentAreaState {
            top_floor: self.top_floor,
            elevator: (self.elevator as i32 + direction) as usize,
            components: self.components.iter().enumerate().map(|(i,c)| {
                if component_indices.contains(&i) {
                    (*c as i32 + direction) as usize
                } else {
                    *c
                }
            }).collect(),
            elements: self.elements.clone(),
            search_dir: self.search_dir.clone()
        }
    }

    fn generate_valid_children(&self) -> Vec<ContainmentAreaState> {
        self.components.iter().enumerate().fold(vec!(), |new_states, (i, floor)| {
            if self.elevator == *floor {
                let can_move_up = *floor < (self.top_floor - 1) && (self.search_dir == SearchDir::Upwards ||
                                                                    (*floor+1..self.top_floor).any(|f| !self.is_floor_empty(f)));
                let up = if can_move_up {
                    vec!(self.permute_components(vec!(i), 1))
                } else {
                    vec!()
                };

                let can_move_down = *floor > 0 && (self.search_dir == SearchDir::Downwards || 
                                                    (0..*floor).any(|f| !self.is_floor_empty(f)));
                let down = if can_move_down {
                    vec!(self.permute_components(vec!(i), -1))
                } else {
                    vec!()
                };

                let pairs = self.components.iter().enumerate().fold(vec!(), |pair_states, (j, f)| {
                    if j > i && f == floor {
                        let up_pair = if can_move_up {
                            vec!(self.permute_components(vec!(i,j),1))
                        } else {
                            vec!()
                        };

                        let down_pair = if can_move_down {
                            vec!(self.permute_components(vec!(i,j),-1))
                        } else {
                            vec!()
                        };

                        pair_states.iter().chain(up_pair.iter()).chain(down_pair.iter()).cloned().collect()               
                    } else {
                        pair_states
                    }
                });

                new_states.iter().chain(up.iter()).chain(down.iter()).chain(pairs.iter()).cloned().collect()
            } else {
                new_states
            }
        }).iter().filter(|s| s.state_is_valid()).cloned().collect()
    }

    fn get_completed_version(&self) -> ContainmentAreaState {
        ContainmentAreaState {
            elevator: self.top_floor - 1,
            components: self.components.iter().map(|_| self.top_floor - 1).collect(),
            elements: self.elements.clone(),
            top_floor: self.top_floor,
            search_dir: SearchDir::Downwards
        }
    }
}

fn evaluate_path(start_input: &str, best_path_so_far: Arc<AtomicUsize>, all_nodes_explored: Arc<AtomicBool>, search_dir: SearchDir) {

    let start = ContainmentAreaState::parse(start_input);
    let goal = start.get_completed_version();

    let (initial_state, search_goal) = match search_dir {
        SearchDir::Upwards => (&start, &goal),
        SearchDir::Downwards => (&goal, &start),
    };

    let mut search_heap = BinaryHeap::new();
    search_heap.push(Node{state: initial_state.clone(), distance: 0, score: 0, search_dir: initial_state.search_dir.clone()});

    let mut seen = HashMap::new();

    while let Some(next_state) = search_heap.pop() {
        if all_nodes_explored.load(Ordering::Relaxed) {
            //Another thread has searched the entire space, just quit
            println!("Another thread has finished, aborting thread {:?}", thread::current().name().unwrap());
            return;
        }

        let current_distance = next_state.distance;

        if best_path_so_far.load(Ordering::Relaxed) <= current_distance {
            continue;
        }

        if next_state.state == *search_goal {
            println!("Thread {:?} found new best path {}", thread::current().name().unwrap(), current_distance);
            best_path_so_far.store(current_distance, Ordering::Relaxed);
            continue;
        }

        seen.insert(next_state.state.clone(), current_distance);

        for child_state in next_state.state.generate_valid_children() {

            let child_distance = current_distance + 1;

            {
                let ref have_seen_state = seen.get(&child_state);
                if have_seen_state.is_some() && *have_seen_state.unwrap() <= child_distance {
                    continue;
                }
            }

            seen.insert(child_state.clone(), child_distance);
            search_heap.push(Node{state: child_state.clone(), distance: child_distance, score: child_state.get_score(), search_dir: child_state.search_dir});
        }
    }

    println!("Thread {:?} finished!", thread::current().name().unwrap());
    all_nodes_explored.store(true, Ordering::Relaxed);
}

fn find_shortest_path(input: &str) -> usize {
    let best_path = Arc::new(AtomicUsize::new(usize::max_value()));
    let thread_finished = Arc::new(AtomicBool::new(false));

    let search_threads = (0..2).map(|thread_id| {
        let best_path_tracker = best_path.clone();
        let thread_finished_tracker = thread_finished.clone();

        let search_dir = if thread_id % 2 == 0 {
            SearchDir::Upwards
        } else {
            SearchDir::Downwards
        };
        let thread_input = input.to_owned();

        thread::Builder::new().name(format!("{:?}", search_dir).to_string()).spawn(move || evaluate_path(&thread_input, best_path_tracker, thread_finished_tracker, search_dir)).unwrap()
    }).collect::<Vec<std::thread::JoinHandle<_>>>();

    for searcher in search_threads {
        let _ = searcher.join();
    }

    best_path.load(Ordering::Relaxed)
}

fn main() {
    println!("Shortest Path: {}", find_shortest_path(include_str!("../input/input.txt")));
}

#[test]
fn parse() {
    let input = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

    let expected = ContainmentAreaState {
        top_floor: 4,
        elevator: 0,
        components: vec!(
            0,
            2,
            0,
            1
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    assert_eq!(expected, ContainmentAreaState::parse(input));
}

#[test]
fn path_gen() {
    let input = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

    assert_eq!(11, find_shortest_path(input)); 
}

#[test]
fn child_generation_step_1() {
    let state = ContainmentAreaState {
        top_floor: 4,
        elevator: 0,
        components: vec!(
            0,
            2,
            0,
            1
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    let expected = ContainmentAreaState {
        top_floor: 4,
        elevator: 1,
        components: vec!(
            0,
            2,
            1,
            1
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_2() {
    let state = ContainmentAreaState {
        top_floor: 4,
        elevator: 1,
        components: vec!(
            0,
            2,
            1,
            1
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    let expected = ContainmentAreaState {
        top_floor: 4,
        elevator: 2,
        components: vec!(
            0,
            2,
            2,
            2
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_3() {
    let state = ContainmentAreaState {
        top_floor: 4,
        elevator: 2,
        components: vec!(
            0,
            2,
            2,
            2
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    let expected = ContainmentAreaState {
        top_floor: 4,
        elevator: 1,
        components: vec!(
            0,
            2,
            1,
            2
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_4() {
    let state = ContainmentAreaState {
        top_floor: 4,
        elevator: 1,
        components: vec!(
            0,
            2,
            1,
            2
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    let expected = ContainmentAreaState {
        top_floor: 4,
        elevator: 0,
        components: vec!(
            0,
            2,
            0,
            2
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_5() {
    let state = ContainmentAreaState {
        top_floor: 4,
        elevator: 0,
        components: vec!(
            0,
            2,
            0,
            2
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    let expected = ContainmentAreaState {
        top_floor: 4,
        elevator: 1,
        components: vec!(
            1,
            2,
            1,
            2
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_6() {
    let state = ContainmentAreaState {
        top_floor: 4,
        elevator: 1,
        components: vec!(
            1,
            2,
            1,
            2
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    let expected = ContainmentAreaState {
        top_floor: 4,
        elevator: 2,
        components: vec!(
            2,
            2,
            2,
            2
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_7() {
    let state = ContainmentAreaState {
        top_floor: 4,
        elevator: 2,
        components: vec!(
            2,
            2,
            2,
            2
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    let expected = ContainmentAreaState {
        top_floor: 4,
        elevator: 3,
        components: vec!(
            3,
            2,
            3,
            2
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_8() {
    let state = ContainmentAreaState {
        top_floor: 4,
        elevator: 3,
        components: vec!(
            3,
            2,
            3,
            2
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    let expected = ContainmentAreaState {
        top_floor: 4,
        elevator: 2,
        components: vec!(
            3,
            2,
            2,
            2
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_9() {
    let state = ContainmentAreaState {
        top_floor: 4,
        elevator: 2,
        components: vec!(
            3,
            2,
            2,
            2
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    let expected = ContainmentAreaState {
        top_floor: 4,
        elevator: 3 ,
        components: vec!(
            3,
            3,
            2,
            3
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_10() {
    let state = ContainmentAreaState {
        top_floor: 4,
        elevator: 3 ,
        components: vec!(
            3,
            3,
            2,
            3
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    let expected = ContainmentAreaState {
        top_floor: 4,
        elevator: 2,
        components: vec!(
            2,
            3,
            2,
            3
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_11() {
    let state = ContainmentAreaState {
        top_floor: 4,
        elevator: 2,
        components: vec!(
            2,
            3,
            2,
            3
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };


    let expected = ContainmentAreaState {
        top_floor: 4,
        elevator: 3,
        components: vec!(
            3,
            3,
            3,
            3
        ),
        elements: vec!(
            "lithium".to_string(),
            "hydrogen".to_string()
        ),
        search_dir: SearchDir::Upwards
    };

    assert!(state.generate_valid_children().contains(&expected));
    assert!(expected == state.get_completed_version());
}