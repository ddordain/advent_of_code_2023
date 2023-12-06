use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashMap;

#[derive(Debug)]
struct Rgb(u64,u64,u64);

fn file_to_vector(filepath: &str) -> Result<Vec<String>, Error> {
    let file = fs::File::open(filepath)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect();
    lines
}

fn build_hashmap(vec: Vec<String>) -> HashMap<u8, Rgb> {
    let mut hashmap : HashMap<u8, Rgb> = HashMap::new();
    let color_regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let id_regex = Regex::new(r"Game (\d+)").unwrap();

    for v in vec {
        if let Some(cap) = id_regex.captures(&v) {
            let game_id : u8 = cap[1].parse().unwrap();
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for color_cap in color_regex.captures_iter(&v) {
                let value: u64 = color_cap[1].parse().unwrap();
                let color = &color_cap[2];

                match color {
                    "red" => max_red = max_red.max(value),
                    "green" => max_green = max_green.max(value),
                    "blue" => max_blue = max_blue.max(value),
                    _ => {}
                }
            }
            hashmap.insert(game_id, Rgb(max_red, max_green, max_blue));
        }
    }
    hashmap
}

fn validate_game(hashmap : HashMap<u8, Rgb>) -> Vec<u64> {
    let mut vec : Vec<u64> = vec![];
    for (key, value) in hashmap.iter() {
        // if (value.0 <= 12) && (value.1 <= 13) && (value.2 <= 14) {
        //     vec.push(*key);
        // }
        let result : u64 = value.0 * value.1 * value.2;
        vec.push(result);
    }
    vec
}

fn main() {
    let filepath = "input.txt";
    let vec = file_to_vector(filepath).unwrap();
    for line in &vec {
        println!("{:#?}", line);
    }
    let hash = build_hashmap(vec);
    for line in &hash {
        println!("{:#?}", line);
    }
    let vec = validate_game(hash);
    for line in &vec {
        println!("{:#?}", line);
    }
    let result : u64 = vec.iter().map(|&n| n as u64).sum();
    println!("the sum of ID is {}", result);
}
