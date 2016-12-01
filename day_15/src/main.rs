use std::io::prelude::*;
use std::fs::File;
use std::cmp;

#[derive(Debug)]
struct Ingredient<'a> {
    name: &'a str,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl<'a> Ingredient<'a> {
    fn ensure_non_negative(&mut self) -> &Ingredient{
		self.capacity = cmp::max(0, self.capacity);
		self.durability = cmp::max(0, self.durability);
		self.flavor = cmp::max(0, self.flavor);
		self.texture = cmp::max(0, self.texture);

		self
	}

	fn get_score(&self) -> i32 {
		if self.calories == 500 {
		    self.capacity * self.durability * self.flavor * self.texture
		} else {
		    0
		}
	}
}

fn main() {
   let mut file = File::open("input/input.txt").unwrap();
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();
   
   let ingredients = get_ingredients(&input);
   let capacities = get_capacities(ingredients.len(), 100);

   println!("best cookie score = {}", capacities.iter().map(|x| get_cookie_score(&ingredients, &x)).max().unwrap());
}

fn get_capacities(num_ingredients: usize, max_teaspoons: u32) -> Vec<Vec<u32>> {	
    if num_ingredients == 1 {
        return vec![vec![max_teaspoons]];
    }

    (0..max_teaspoons+1).flat_map(|x| {
        let mut retval = get_capacities(num_ingredients - 1, max_teaspoons - x);

        for combination in &mut retval {
            combination.push(x);
        }
        retval
    }).collect::<Vec<_>>()
}

fn get_cookie_score(ingredients: &Vec<Ingredient>, capacities: &Vec<u32>) -> i32 {
	let mut mixed_ingredients = Ingredient { name: "", capacity: 0, durability: 0, flavor: 0, texture: 0, calories: 0 };
	for (i, teaspoon) in capacities.iter().enumerate() {
	    mixed_ingredients.capacity += *teaspoon as i32 * (ingredients[i].capacity);
		mixed_ingredients.durability += *teaspoon as i32 * (ingredients[i].durability);
		mixed_ingredients.flavor += *teaspoon as i32 * (ingredients[i].flavor);
		mixed_ingredients.texture += *teaspoon as i32 * (ingredients[i].texture);
		mixed_ingredients.calories += *teaspoon as i32 * (ingredients[i].calories);
	}

	mixed_ingredients.ensure_non_negative().get_score()
}

fn get_ingredients(input: &String) -> Vec<Ingredient> {
    input.lines().into_iter().map(|l| {
    	let split: Vec<&str> = l.split(|c| c == ' ' || c == ':' || c == ',').collect();
        Ingredient {
        	name: split[0], 
        	capacity: split[3].parse::<i32>().unwrap(),
        	durability: split[6].parse::<i32>().unwrap(),
        	flavor: split[9].parse::<i32>().unwrap(),
        	texture: split[12].parse::<i32>().unwrap(),
        	calories: split[15].parse::<i32>().unwrap()
        }
    }).collect()
}