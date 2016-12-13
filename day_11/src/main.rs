//use std::fmt;

#[derive(Debug, PartialEq, Clone)]
enum Occupier {
    Microchip {microchip_type: String},
    Generator {generator_type: String},
    Empty,
}

#[derive(Debug, PartialEq, Clone)]
struct Floor {
    contents: Vec<Occupier>
}

#[derive(Debug,PartialEq, Clone)]
struct ContainmentArea {
    elevator_index: usize,
    floors: Vec<Floor>
}

/*impl fmt::Debug for ContainmentArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.render_diagram())
    }
}*/

impl ContainmentArea {
    /*fn render_diagram(&self) -> String {
        //max number of components
        let num_components = self.floors.iter().fold(0, |num, floor| num + floor.contents.len() );
        self.floors.iter().rev().enumerate().map(|floor| {

        }).collect()
    }*/

    fn is_completed_state(&self) -> bool {
        self.elevator_index == self.floors.len() - 1 && self.floors.iter().take(self.floors.len() - 10).all(|ref f| f.contents.is_empty() )
    }

    /*fn is_possible_move(&self, moving_component: &Occupier, to_floor: usize) -> bool {
        let ref target_floor_contents = self.floors[to_floor].contents;
        target_floor_contents.is_empty() || match moving_component {
            &Occupier::Microchip{ref microchip_type} => {
                //Can only move a microchip to a floor that either has no generators or a generator of the matching type
                !target_floor_contents.iter().any(|component| {
                    match component {
                        &Occupier::Generator{ref generator_type} => generator_type == microchip_type,
                        _ => true
                    }
                })
            },
            &Occupier::Generator{ref generator_type} => {
                //A generator can be on any floor, as long as it doesn't fry any microchips
                !target_floor_contents.iter().any(|component| {
                    match component {
                        &Occupier::Microchip{ref microchip_type} => generator_type == microchip_type,
                        _ => true,
                    }
                })
            }
        }
    }*/

    fn does_floor_contain_generator(&self, floor_index: usize, of_type: &str) -> bool {
        for component in &self.floors[floor_index].contents {
            match component {
                &Occupier::Generator{ref generator_type} => if generator_type == of_type { return true },
                _ => continue,
            }
        }

        return false;
    }

    fn does_floor_contain_unpowered_microchip(&self, floor_index: usize) -> bool {
        for component in &self.floors[floor_index].contents {
            match component {
                &Occupier::Microchip{ref microchip_type} => if !self.does_floor_contain_generator(floor_index, microchip_type) { return true },
                _ => continue,
            }
        }

        return false;
    }

    fn is_possible_move(&self, moving_components: (&Occupier, &Occupier), to_floor: usize) -> bool {
        //Firstly, can they be moved together?
        if !is_valid_pairing(moving_components) {
            return false;
        }

        //Secondly, can they co-exist with everything on the target floor?
        //If this is a microchip/generator pair, then they can co-exist with anything *except* an unpowered microchip

        false
    }

    fn generate_possible_moves_to_floor(&self, to_floor: usize) -> Vec<ContainmentArea> {
        let ref current_floor_contents = self.floors[self.elevator_index].contents;

        vec!()
    }

    fn generate_possible_moves(&self) -> Vec<ContainmentArea> {
        //Nothing on this floor? Then we're in a pretty shit position as we have to have something to move the elevator
        if self.floors[self.elevator_index].contents.is_empty() {
            return vec!();
        }

        let moves_down = if self.elevator_index > 0 {
            self.generate_possible_moves_to_floor(self.elevator_index - 1)
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

        moves_down.iter().chain(moves_up.iter()).cloned().collect()
    }
}

fn is_generator_microchip_match(component_pair: (&Occupier, &Occupier)) -> bool {
    match component_pair.0 {
        &Occupier::Microchip{ref microchip_type} => match component_pair.1 {
            &Occupier::Generator{ref generator_type} => microchip_type == generator_type,
            _ => false
        },
        &Occupier::Generator{ref generator_type} => match component_pair.1 {
            &Occupier::Microchip{ref microchip_type} => microchip_type == generator_type,
            _ => false
        },
        _ => false
    }
}

fn is_valid_pairing(component_pair: (&Occupier, &Occupier)) -> bool {
    //two occupiers can occupy the same space (floor or elevator) if
    //a) It's a microchip with it's paired generator, so a protective shield is generated
    //b) They're both generators
    //c) They're both microchips
    //d) One of the occupants is 'empty'

    is_generator_microchip_match(component_pair) || 
    match component_pair.0 {
        &Occupier::Microchip{ref microchip_type} => match component_pair.1 {
            &Occupier::Microchip{ref microchip_type} => true,
            &Occupier::Empty => true,
            _ => false
        },
        &Occupier::Generator{ref generator_type} => match component_pair.1 {
            &Occupier::Generator{ref generator_type} => true,
            &Occupier::Empty => true,
            _ => false
        },
        _ => true
    }
}

fn parse_input(input: &str) -> ContainmentArea {
    input.lines().fold(ContainmentArea{elevator_index: 0, floors: vec!()}, |area, line| {
        let new_floor = match line.contains("nothing relevant") {
            true => Floor {contents: vec!()},
            false => {
                let floor_generators = line.rmatch_indices("generator").fold(vec!(), |generators, (i, _)| {
                    generators.iter().chain(vec!(Occupier::Generator{ generator_type: line[0..i].split_whitespace().last().unwrap().to_string() }).iter()).cloned().collect()
                });

                let floor_microchips = line.rmatch_indices("microchip").fold(vec!(), |generators, (i, _)| {
                    generators.iter().chain(vec!(Occupier::Microchip{ microchip_type: line[0..i].split_whitespace().last().unwrap().split('-').nth(0).unwrap().to_string() }).iter()).cloned().collect()
                });

                Floor {contents: floor_generators.iter().chain(floor_microchips.iter()).cloned().collect()}
            },
        };

        let new_floor_chain = area.floors.iter().chain(vec!(new_floor).iter()).cloned().collect::<Vec<Floor>>();
        ContainmentArea {elevator_index: area.elevator_index, floors: new_floor_chain.to_vec()}
    })
}

fn main() {
    let initial_state = parse_input(include_str!("../input/input.txt"));
    println!("{:#?}", initial_state);

    println!("Possible next states: {:#?}", initial_state.generate_possible_moves());
}

#[test]
fn parse() {
    let input = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

    let expected = ContainmentArea {
        elevator_index: 0,
        floors: vec!(
                Floor {
                    contents: vec!(Occupier::Microchip{microchip_type: "lithium".to_string()}, Occupier::Microchip{microchip_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!(Occupier::Generator{generator_type: "hydrogen".to_string()})
                },
                Floor {
                    contents: vec!(Occupier::Generator{generator_type: "lithium".to_string()})
                },
                Floor {
                    contents: vec!()
                }
            )
    };

    let result = parse_input(input);

    assert_eq!(expected, result);
}