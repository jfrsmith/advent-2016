extern crate rustc_serialize;

use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::json::{Json, Object, Array};

fn main() {
   let mut file = File::open("input/input.txt").unwrap();
   let mut input = String::new();
   file.read_to_string(&mut input).unwrap();

   let input = Json::from_str(&input).unwrap();
   let root = input.as_object().unwrap();

   let total = parse_object(root, false);
   
   println!("{}", total);
}

fn has_red(obj: &Object) -> bool {
    for (_, value) in obj.iter() {
    	match value.as_string() {
    	    Some(val) => {
    	    	if val == "red" {
    	    	    return true;
    	    	}
    	    },
    	    None => {},
    	}
    }

    false
}

fn parse_object(obj: &Object, check_for_red: bool) -> i32 {
	let mut total: i32 = 0;

	if check_for_red && has_red(obj) {
	    return 0;
	}

	for (_, value) in obj.iter() {
    	match *value {
            Json::U64(v) => total += v as i32,
            Json::I64(v) => total += v as i32,
            Json::F64(v) => total += v as i32,
            Json::Array(ref v) => total += parse_array(&v),
            Json::Object(ref v) => total += parse_object(&v, true),
            _ => {}
        }
    }

    total as i32
}

fn parse_array(arr: &Array) -> i32 {
	let mut total: i32 = 0;

    for val in arr {
    	if val.is_number() {
    	    total += val.as_i64().unwrap() as i32;
    	} else if val.is_object() {
    	    total += parse_object(val.as_object().unwrap(), true);
    	} else if val.is_array() {
    		total += parse_array(val.as_array().unwrap());
    	}
    }

    total
}