extern crate hex;

use std::collections::HashMap;
use std::str;
use std::sync::OnceLock;
use std::{env, fs};
use str::from_utf8;

// XORs a string with the same byte
fn single_xor(input: &str, xor: u8) -> String {
    let mut result: Vec<u8> = Vec::new();
    hex::from_hex(&input).into_iter().map(|c| c ^ xor).for_each(|c| result.push(c));
    from_utf8(&result).unwrap().to_owned()
}

// This is a normalised frequency dictionary of some of the most used
// characters in the english language, starting with the space character.
// The values are normalised, meaning that space will appear aprox. 8 times
// more often than the letter u.
fn get_freq_dictionary() -> &'static HashMap<char, i16> {
    let contents = fs::read_to_string("src/frequency.txt").expect("file doesn't exist");
    static HASHMAP: OnceLock<HashMap<char, i16>> = OnceLock::new();
    HASHMAP.get_or_init(||
        contents.lines().map(|line| line.as_bytes()).map(|line| (
            line[0] as char,
            from_utf8(&line[2..]).map(|s| s.parse::<i16>().unwrap()).unwrap())).collect()
    )
}

// scores a piece of string based on the relative frequency of its characters
fn score_it(input: String) -> i16 {
    input.chars().map(|c| get_freq_dictionary().get(&c).unwrap_or(&0)).sum()
}
const DICTIONARY: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
fn main() {
    let args: Vec<String> = env::args().collect();

    // loop through all potential character-solutions and score the solutions based on the
    // frequency histogram - the one with the max value is the result.
    let result = DICTIONARY.chars()
        .into_iter()
        .map(|c| (c, score_it(single_xor(&args[1], c as u8))))
        .max_by_key(|c| c.1).unwrap();

    println!("Solution is {:?} with score {:?}", result.0, result.1);
    println!("Output is {:?}", single_xor(&args[1], result.0 as u8));
}
