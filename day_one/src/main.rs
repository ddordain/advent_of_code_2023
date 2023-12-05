use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader, Error};

fn file_to_vector(filepath: &str) -> Result<Vec<String>, Error> {
    let file = fs::File::open(filepath)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect();
    lines
}

fn convert_vec_string_to_digit_string(vec: Vec<String>) -> Option<Vec<String>> {
    let vec_digit = vec
        .iter()
        .map(|s| s.chars().filter(|c| c.is_digit(10)).collect())
        .collect();
    Some(vec_digit)
}

//already filter out without digit
fn transform_strings_to_u8(vec: Vec<String>) -> Option<Vec<u8>> {
    let vec_u8 = vec
        .iter()
        .filter_map(|s| {
            if let Some(first) = s.chars().next() {
                let last = s.chars().last().unwrap_or(first);
                let mut combined = String::new();
                combined.push(first);
                combined.push(last);
                combined.parse::<u8>().ok()
            } else {
                Some(0)
            }
        })
        .collect();
    Some(vec_u8)
}

//this regex trick assumes no twoneight ^^ XD
//but i was lucky :D
fn convert_words_to_digits(vec: Vec<String>) -> Option<Vec<String>> {
    let patterns = [
        (Regex::new(r"twone").unwrap(), "21"),
        (Regex::new(r"oneight").unwrap(), "18"),
        (Regex::new(r"eightwo").unwrap(), "82"),
        (Regex::new(r"nineight").unwrap(), "98"),
        (Regex::new(r"eighthree").unwrap(), "83"),
        (Regex::new(r"sevenine").unwrap(), "79"),
        (Regex::new(r"one").unwrap(), "1"),
        (Regex::new(r"two").unwrap(), "2"),
        (Regex::new(r"three").unwrap(), "3"),
        (Regex::new(r"four").unwrap(), "4"),
        (Regex::new(r"five").unwrap(), "5"),
        (Regex::new(r"six").unwrap(), "6"),
        (Regex::new(r"seven").unwrap(), "7"),
        (Regex::new(r"eight").unwrap(), "8"),
        (Regex::new(r"nine").unwrap(), "9"),
    ];

    let vec_replace = vec
        .iter()
        .map(|s| {
            let mut result = s.clone();
            for (pattern, replacement) in patterns.iter() {
                result = pattern.replace_all(&result, *replacement).into_owned();
            }
            result
        })
        .collect();
    Some(vec_replace)
}

fn main() {
    let filepath = "input.txt";
    let lines = file_to_vector(filepath).unwrap();
    // for v in &lines {
    //     println!("{}", v);
    // }
    let vec = convert_words_to_digits(lines).unwrap();
    // for v in &vec {
    //     println!("{}", v);
    // }
    let vec = convert_vec_string_to_digit_string(vec).unwrap();
    // for v in &vec {
    //     println!("{}", v);
    // }
    let vec = transform_strings_to_u8(vec).unwrap();
    // for v in &vec {
    //     println!("{:?}", v);
    // }
    let sum: u32 = vec.iter().map(|&n| n as u32).sum();
    println!("the Sum is {}", sum);
}
