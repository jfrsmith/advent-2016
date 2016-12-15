use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
enum Component {
    Microchip {microchip_type: String},
    Generator {generator_type: String}
}

impl fmt::Debug for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &Component::Microchip{ref microchip_type} => format!("{}{}", microchip_type.to_uppercase().chars().nth(0).unwrap(), "M"),
            &Component::Generator{ref generator_type} => format!("{}{}", generator_type.to_uppercase().chars().nth(0).unwrap(), "G")
        })
    }
}

#[derive(Debug, Clone, Hash)]
struct Floor {
    contents: Vec<Component>
}

impl Eq for Floor {}

impl PartialEq for Floor {
    fn eq(&self, other: &Floor) -> bool {
        let mut v1 = self.contents.clone();
        let mut v2 = other.contents.clone();
        v1.sort();
        v2.sort();
        v1 == v2
    }
}

impl Floor {
    fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    fn is_valid(&self) -> bool {
        //A floor is valid if it
        //1. Consists of a single type of component
        //2. Consists of only matched generator-microchip pairs and/or unpaired generators
        //3. Is empty

        if self.contents.is_empty() ||
            self.contents.iter().by_ref().all(|x| match x {&Component::Microchip{..} => true, _ => false}) ||
            self.contents.iter().by_ref().all(|x| match x {&Component::Generator{..} => true, _ => false}) {
                return true;
        }

        for c in &self.contents {
            let is_valid_occupant = match c {
                &Component::Microchip{ref microchip_type} => self.contains_generator(&microchip_type),
                _ => true,
            };

            if !is_valid_occupant {
                return false;
            }
        }

        return true;
    }

    fn contains_generator(&self, of_type: &str) -> bool {
        for component in &self.contents {
            match component {
                &Component::Generator{ref generator_type} => if generator_type == of_type { return true },
                _ => continue,
            }
        }

        return false;
    }

    fn get_component_pairs(&self) -> Vec<(Component, Component)> {
        let outer_pairs = self.contents.iter().enumerate().fold(vec!(), | pairs, (i, c1) | {
            let inner_pairs = self.contents.iter().enumerate().filter_map(|(j,c2)| {
                if j != i && is_valid_pairing((c1,c2)) {
                    Some((c1.clone(), c2.clone()))
                } else {
                    None
                }
            }).collect::<Vec<(Component, Component)>>();

            pairs.iter().chain(inner_pairs.iter()).cloned().collect()
        });

        //trim repeated reverse pairs
        outer_pairs.iter().enumerate().filter_map(|(i,p)| {
        if outer_pairs[i..].contains(&(p.1.clone(), p.0.clone())) { None } else { Some((p.0.clone(), p.1.clone())) }
        }).collect::<Vec<(Component, Component)>>()
    }
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct ContainmentAreaState {
    elevator_index: usize,
    floors: Vec<Floor>
}

impl fmt::Debug for ContainmentAreaState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.output())
    }
}

impl ContainmentAreaState {
    fn output(&self) -> String {
        let num_components = self.get_num_components();
        self.floors.iter().enumerate().rev().fold("".to_string(), |out_str, (i, floor)| {
            let contents_str = (0..num_components).map(|comp_index| {
                if comp_index < floor.contents.len() { 
                    format!("{:?}", floor.contents[comp_index])
                } else {
                    ". ".to_string()
                }
            }).collect::<Vec<String>>().join(" ");

            out_str + &format!("F{} {} {}", i+1, if self.elevator_index == i {"E"} else {"."}, contents_str) + if i != 0 {"\n"} else {""}
        })
    }

    fn get_num_components(&self) -> usize {
        self.floors.iter().fold(0, |cnt, floor| {
            cnt + floor.contents.len()
        })
    }

    fn is_completed_state(&self) -> bool {
        self.elevator_index == self.floors.len() - 1 && self.floors.iter().take(self.floors.len() - 1).all(|ref f| f.contents.is_empty() )
    }

    fn try_generate_permutation(&self, move_components:&Vec<Component>, from_floor: &Floor, to_floor: &Floor, new_elevator_index: usize) -> Option<ContainmentAreaState> {        
        //If we remove these occupants, will the current floor no longer be valid?
        let removed = Floor{ contents: from_floor.contents.iter().filter(|x| !move_components.contains(x) ).cloned().collect::<Vec<Component>>() };
        if !removed.is_valid() {
            return None;
        }

        //If we add these occupants to the other floor, with the other floor no longer be valid?
        let added = Floor { contents: to_floor.contents.iter().chain(move_components.iter()).cloned().collect::<Vec<Component>>() };
        if !added.is_valid() {
            return None;
        }

        //valid to move this item from current floor to target floor, let's build the new state
        let new_state = Some(ContainmentAreaState {
            elevator_index: new_elevator_index,
            floors: (0..self.floors.len()).map(|i| {
                if i == self.elevator_index {
                    removed.clone()
                }
                else if i == new_elevator_index {
                    added.clone()
                }
                else {
                    self.floors[i].clone()
                }
            }).collect::<Vec<Floor>>()
        });

        new_state
    }

    fn generate_possible_moves_to_floor(&self, to_floor: usize) -> Vec<ContainmentAreaState> {
        let ref current_floor = self.floors[self.elevator_index];
        let ref new_floor = self.floors[to_floor];

        let ref current_floor_contents = current_floor.contents;

        //Firstly, let's just move each content individually to the new floor
        let single_move_permutations = current_floor_contents.iter().fold(vec!(), |perms, occupant| {
            let new_permutation = self.try_generate_permutation(&vec!(occupant.clone()), current_floor, new_floor, to_floor);
            if new_permutation.is_some() {
                perms.iter().chain(vec!(new_permutation.unwrap()).iter()).cloned().collect::<Vec<ContainmentAreaState>>()
            }
            else {
                perms
            }   
        });

        //Secondly, generate all unique pairs from the vec
        let double_move_permutations = current_floor.get_component_pairs().iter().fold(vec!(), |perms, occupant_pair| {
            let new_permutation = self.try_generate_permutation(&vec!(occupant_pair.0.clone(), occupant_pair.1.clone()), current_floor, new_floor, to_floor);
            if new_permutation.is_some() {
                perms.iter().chain(vec!(new_permutation.unwrap()).iter()).cloned().collect::<Vec<ContainmentAreaState>>()
            }
            else {
                perms
            }  
        });

        single_move_permutations.iter().chain(double_move_permutations.iter()).cloned().collect::<Vec<ContainmentAreaState>>()
    }

    fn state_is_valid(&self) -> bool {
        self.floors[self.elevator_index].is_valid()
    }

    fn generate_valid_children(&self) -> Vec<ContainmentAreaState> {

        //Is this state the final state?
        if self.is_completed_state() {
            return vec!();
        }

        //Nothing on this floor? Then we're in a pretty shit position as we have to have something to move the elevator
        if self.floors[self.elevator_index].contents.is_empty() {
            panic!("Nothing to move from state! {:#?}", self);
        }

        let moves_down = if self.elevator_index > 0 {
            if self.floors[0..self.elevator_index].iter().by_ref().any(|x| !x.is_empty()) {
                self.generate_possible_moves_to_floor(self.elevator_index - 1)
            }
            else {
                vec!()
            }
        }
        else {
            vec!()
        };

        let moves_up = if self.elevator_index < self.floors.len() - 1 {
            self.generate_possible_moves_to_floor(self.elevator_index + 1)
        }
        else {
            vec!()
        };

        if moves_down.is_empty() && moves_up.is_empty() {
            return vec!();
        }

        //Prune these states
        moves_down.iter().chain(moves_up.iter()).filter(|s| s.state_is_valid()).cloned().collect()
    }
}

struct Node {
    state: ContainmentAreaState,
    distance: i32
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

fn is_generator_microchip_match(component_pair: (&Component, &Component)) -> bool {
    match component_pair.0 {
        &Component::Microchip{ref microchip_type} => match component_pair.1 {
            &Component::Generator{ref generator_type} => microchip_type == generator_type,
            _ => false
        },
        &Component::Generator{ref generator_type} => match component_pair.1 {
            &Component::Microchip{ref microchip_type} => microchip_type == generator_type,
            _ => false
        }
    }
}

fn is_valid_pairing(component_pair: (&Component, &Component)) -> bool {
    //two Components can occupy the elevator if
    //a) It's a microchip with it's paired generator, so a protective shield is generated
    //b) They're both generators
    //c) They're both microchips

    is_generator_microchip_match(component_pair) || 
    match component_pair.0 {
        &Component::Microchip{..} => match component_pair.1 {
            &Component::Microchip{..} => true,
            _ => false
        },
        &Component::Generator{..} => match component_pair.1 {
            &Component::Generator{..} => true,
            _ => false
        }
    }
}

fn parse_input(input: &str) -> ContainmentAreaState {
    input.lines().fold(ContainmentAreaState{elevator_index: 0, floors: vec!()}, |area, line| {
        let new_floor = match line.contains("nothing relevant") {
            true => Floor {contents: vec!()},
            false => {
                let floor_generators = line.rmatch_indices("generator").fold(vec!(), |generators, (i, _)| {
                    generators.iter().chain(vec!(Component::Generator{ generator_type: line[0..i].split_whitespace().last().unwrap().to_string() }).iter()).cloned().collect()
                });

                let floor_microchips = line.rmatch_indices("microchip").fold(vec!(), |generators, (i, _)| {
                    generators.iter().chain(vec!(Component::Microchip{ microchip_type: line[0..i].split_whitespace().last().unwrap().split('-').nth(0).unwrap().to_string() }).iter()).cloned().collect()
                });

                Floor {contents: floor_generators.iter().chain(floor_microchips.iter()).cloned().collect()}
            },
        };

        let new_floor_chain = area.floors.iter().chain(vec!(new_floor).iter()).cloned().collect::<Vec<Floor>>();
        ContainmentAreaState {elevator_index: area.elevator_index, floors: new_floor_chain.to_vec()}
    })
}

fn get_shortest_path_to_completion(initial_state: &ContainmentAreaState) -> i32 {
    let mut search_heap = BinaryHeap::new();
    search_heap.push(Node {state: initial_state.clone(), distance: 0});

    let mut seen_heap = HashMap::new();
    while let Some(next_state) = search_heap.pop() {
        let current_distance = next_state.distance;

        seen_heap.insert(next_state.state.clone(), current_distance);

       // println!("\nEvaluating state:\n{:?}\nDistance = {}", next_state.state, current_distance);

        if next_state.state.is_completed_state() {
            //println!("=========== Is Final State! ===========");
            break;
        }

        //println!("Generated Children: {:?}", next_state.state.generate_valid_children());

        for child_state in next_state.state.generate_valid_children() {

            let child_distance = current_distance + 1;

            {
                let have_seen_state = seen_heap.get(&child_state);
                if have_seen_state.is_some() && *have_seen_state.unwrap() <= child_distance {
                    continue;
                }
            }

            search_heap.push(Node {state: child_state.clone(), distance: child_distance});
        }

        /*for queued in &search_heap {
            println!("\nQueue Entry:\n{:?} Distance: {}", queued.state, queued.distance);
        }*/
    }

    let (_, distance) = seen_heap.iter().filter(|&(key,_)| key.is_completed_state()).nth(0).unwrap();

    *distance as i32
}

fn main() {
    println!("Shortest Path: {}", get_shortest_path_to_completion(&parse_input(include_str!("../input/input.txt"))));
}

#[test]
fn parse() {
    let input = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

    let expected = ContainmentAreaState {
        elevator_index: 0,
        floors: vec!(
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "lithium".to_string()}, Component::Microchip{microchip_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()})
                },
                Floor {
                    contents: vec!()
                }
            )
    };

    let result = parse_input(input);

    assert_eq!(expected, result);
}

#[test]
fn path_gen() {
    let input = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

    assert_eq!(11, get_shortest_path_to_completion(&parse_input(input))); 
}

#[test]
fn child_generation_step_1() {
    let state = ContainmentAreaState {
        elevator_index: 0,
        floors: vec!(
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "lithium".to_string()}, Component::Microchip{microchip_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()})
                },
                Floor {
                    contents: vec!()
                }
            )
    };

    let expected = ContainmentAreaState {
        elevator_index: 1,
        floors: vec!(
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "lithium".to_string()})
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "hydrogen".to_string()}, Component::Microchip{microchip_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()})
                },
                Floor {
                    contents: vec!()
                }
            )
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_2() {
    let state = ContainmentAreaState {
        elevator_index: 1,
        floors: vec!(
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "lithium".to_string()})
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "hydrogen".to_string()}, Component::Microchip{microchip_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()})
                },
                Floor {
                    contents: vec!()
                }
            )
    };

    let expected = ContainmentAreaState {
        elevator_index: 2,
        floors: vec!(
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "lithium".to_string()})
                },
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()}, Component::Microchip{microchip_type: "hydrogen".to_string()}, Component::Generator{generator_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!()
                }
            )
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_3() {
    let state = ContainmentAreaState {
        elevator_index: 2,
        floors: vec!(
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "lithium".to_string()})
                },
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()}, Component::Microchip{microchip_type: "hydrogen".to_string()}, Component::Generator{generator_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!()
                }
            )
    };

    let expected = ContainmentAreaState {
        elevator_index: 1,
        floors: vec!(
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "lithium".to_string()})
                },
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()}, Component::Generator{generator_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!()
                }
            )
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_4() {
    let state = ContainmentAreaState {
        elevator_index: 1,
        floors: vec!(
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "lithium".to_string()})
                },
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()}, Component::Generator{generator_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!()
                }
            )
    };

    let expected = ContainmentAreaState {
        elevator_index: 0,
        floors: vec!(
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "hydrogen".to_string()}, Component::Microchip{microchip_type: "lithium".to_string()})
                },
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()}, Component::Generator{generator_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!()
                }
            )
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_5() {
    let state = ContainmentAreaState {
        elevator_index: 0,
        floors: vec!(
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "lithium".to_string()}, Component::Microchip{microchip_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()}, Component::Generator{generator_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!()
                }
            )
    };

    let expected = ContainmentAreaState {
        elevator_index: 1,
        floors: vec!(
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "hydrogen".to_string()}, Component::Microchip{microchip_type: "lithium".to_string()})
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()}, Component::Generator{generator_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!()
                }
            )
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_6() {
    let state = ContainmentAreaState {
        elevator_index: 1,
        floors: vec!(
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "hydrogen".to_string()}, Component::Microchip{microchip_type: "lithium".to_string()})
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()}, Component::Generator{generator_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!()
                }
            )
    };

    let expected = ContainmentAreaState {
        elevator_index: 2,
        floors: vec!(
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()}, Component::Generator{generator_type: "hydrogen".to_string()}, Component::Microchip{microchip_type: "lithium".to_string()}, Component::Microchip{microchip_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!()
                }
            )
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_7() {
    let state = ContainmentAreaState {
        elevator_index: 2,
        floors: vec!(
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()}, Component::Generator{generator_type: "hydrogen".to_string()}, Component::Microchip{microchip_type: "lithium".to_string()}, Component::Microchip{microchip_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!()
                }
            )
    };

    let expected = ContainmentAreaState {
        elevator_index: 3,
        floors: vec!(
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()}, Component::Generator{generator_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "hydrogen".to_string()}, Component::Microchip{microchip_type: "lithium".to_string()})
                }
            )
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_8() {
    let state = ContainmentAreaState {
        elevator_index: 3,
        floors: vec!(
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()}, Component::Generator{generator_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "hydrogen".to_string()}, Component::Microchip{microchip_type: "lithium".to_string()})
                }
            )
    };

    let expected = ContainmentAreaState {
        elevator_index: 2,
        floors: vec!(
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()}, Component::Generator{generator_type: "hydrogen".to_string()}, Component::Microchip{microchip_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "lithium".to_string()})
                }
            )
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_9() {
    let state = ContainmentAreaState {
        elevator_index: 2,
        floors: vec!(
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "lithium".to_string()}, Component::Generator{generator_type: "hydrogen".to_string()}, Component::Microchip{microchip_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "lithium".to_string()})
                }
            )
    };

    let expected = ContainmentAreaState {
        elevator_index: 3,
        floors: vec!(
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "lithium".to_string()}, Component::Generator{generator_type: "hydrogen".to_string()}, Component::Generator{generator_type: "lithium".to_string()})
                }
            )
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_10() {
    let state = ContainmentAreaState {
        elevator_index: 3,
        floors: vec!(
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "lithium".to_string()}, Component::Generator{generator_type: "hydrogen".to_string()}, Component::Generator{generator_type: "lithium".to_string()})
                }
            )
    };

    let expected = ContainmentAreaState {
        elevator_index: 2,
        floors: vec!(
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "hydrogen".to_string()}, Component::Microchip{microchip_type: "lithium".to_string()})
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "hydrogen".to_string()}, Component::Generator{generator_type: "lithium".to_string()})
                }
            )
    };

    assert!(state.generate_valid_children().contains(&expected));
}

#[test]
fn child_generation_step_11() {
    let state = ContainmentAreaState {
        elevator_index: 2,
        floors: vec!(
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!(Component::Microchip{microchip_type: "hydrogen".to_string()}, Component::Microchip{microchip_type: "lithium".to_string()})
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "hydrogen".to_string()}, Component::Generator{generator_type: "lithium".to_string()})
                }
            )
    };

    let expected = ContainmentAreaState {
        elevator_index: 3,
        floors: vec!(
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!()
                },
                Floor {
                    contents: vec!(Component::Generator{generator_type: "hydrogen".to_string()}, Component::Generator{generator_type: "lithium".to_string()}, Component::Microchip{microchip_type: "lithium".to_string()}, Component::Microchip{microchip_type: "hydrogen".to_string()})
                }
            )
    };

    assert!(state.generate_valid_children().contains(&expected));
}