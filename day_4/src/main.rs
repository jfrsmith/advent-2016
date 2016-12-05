use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug)]
struct RoomLetter(char,u32);
struct Room(String,u32,String,String);

fn generate_checksum(room_name: &str) -> String {
    let mut char_map : HashMap<char, u32> = HashMap::new();
    for c in room_name.chars() {
        let count = match char_map.get(&c) {
            Some(x) => x + 1,
            None => 1
        };
        char_map.insert(c, count);
    }

    let mut checksum_vec : Vec<RoomLetter> = char_map.iter().map(|(c,x)| {
        RoomLetter(*c,*x)
    }).collect();

    checksum_vec.sort_by(|a,b|{
        match b.1.cmp(&a.1) {
            Ordering::Equal => a.0.cmp(&b.0),
            x => x
        }
    });

    checksum_vec.iter().map(|x| x.0).take(5).collect()
}

fn parse_room(entry: &str) -> Room {
    let room_split : Vec<&str> = entry.rsplit(|c| c == ']' || c == '[' || c == '-').collect();
    Room(room_split.split_at(3).1.iter().flat_map(|s| s.chars()).rev().collect(), 
            room_split[2].parse().unwrap(), 
            room_split[1].to_string(),
            entry.to_string())
}

fn get_real_rooms(encrypted_data: &str) -> Vec<Room> {
    encrypted_data.lines().filter_map(|line| {
        let room = parse_room(line);
        match room.2 == generate_checksum(&room.0) {
            true => Some(room),
            false => None
        }
    }).collect()
}

fn get_sector_id_total(input: &str) -> u32 {
    get_real_rooms(input).iter().fold(0, | sector_id_total, room | {
        sector_id_total + room.1
    })
}

fn shift_cipher(shift_char: &char, shift_by: &u32) -> char {
    match shift_char {
        &'-' => ' ',
        &x => {
            let (low, high) = ('a' as u32, 'z' as u32);
            let range = high - low + 1;  
            let shift = (*shift_by % range) as u8;
            let normalised_char = ((x as u8 - low as u8) + shift) % range as u8;
            (normalised_char + low as u8) as char
        }
    }
}

fn get_real_room_name(encrypted_room: &str, sector_id: &u32) -> String {
    let real_name : String = encrypted_room.split(char::is_numeric).collect::<Vec<&str>>()[0].chars().map(|c| shift_cipher(&c, sector_id)).collect();
    real_name.trim().to_string()
}

fn get_north_pole_storage_sector_id(input: &str) -> u32 {
    get_real_rooms(input).iter().filter_map(|room| {
        match get_real_room_name(&room.3, &room.1) == "northpole object storage" {
            true => Some(room.1),
            false => None
        }
    }).collect::<Vec<u32>>()[0]
}

fn main() {
    println!("Sum of Sector IDs = {}", get_sector_id_total(include_str!("../input/input.txt")));
    println!("northpole object storage sector ID = {}", get_north_pole_storage_sector_id(include_str!("../input/input.txt")));
}

#[test]
fn part_one() {
    let inputs = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";
    assert_eq!(1514, get_sector_id_total(&inputs));
}

#[test]
fn part_two() {
    let inputs = "qzmt-zixmtkozy-ivhz";
    assert_eq!("very encrypted name", &get_real_room_name(inputs, &343));
}