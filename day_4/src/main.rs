extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn is_valid_hash(hash: &str) -> bool {
	return &hash[..6] == "000000"
}

fn main() {
    let input = "ckczppom";
	
	let mut num: i64 = 0;
    let mut md5 = Md5::new();
    let mut found = false;

    while !found {
    	num += 1;
    	md5.reset();
        md5.input_str(&format!("{}{}", &input, num));
        found = is_valid_hash(&md5.result_str());
    }

    println!("Found!: {}", num);
}
