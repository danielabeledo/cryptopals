extern crate hex;

use std::collections::HashMap;
use std::fs;
use std::str;
use std::str::from_utf8;
use std::sync::OnceLock;

// XORs a string with the same byte
fn single_xor(input: &str, xor: u8) -> String {
    let mut result: Vec<u8> = Vec::new();
    hex::from_hex(&input).into_iter().map(|c| c ^ xor).for_each(|c| result.push(c));
    from_utf8(&result).unwrap_or("").to_owned()
}

// This is a normalised frequency dictionary of some of the most used
// characters in the english language, starting with the space character.
// The values are normalised, meaning that space will appear aprox. 8 times
// more often than the letter u.
fn get_freq_dictionary() -> &'static HashMap<char, i16> {
    static HASHMAP: OnceLock<HashMap<char, i16>> = OnceLock::new();
    HASHMAP.get_or_init(||
        fs::read_to_string("src/frequency.txt").expect("file doesn't exist")
            .lines().map(|line| line.as_bytes()).map(|line| (
            line[0] as char,
            from_utf8(&line[2..]).map(|s| s.parse::<i16>().unwrap()).unwrap())).collect()
    )
}

// scores a piece of string based on the relative frequency of its characters
fn score_it(input: String) -> i16 {
    input.chars().map(|c| get_freq_dictionary().get(&c).unwrap_or(&0)).sum()
}

const DICTIONARY: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

fn main() {
    let file = fs::read_to_string("./src/4.txt").expect("file not found");

    let result = file.lines()
        .map(|line| (line, DICTIONARY.chars().into_iter()
            .map(|c| (c, score_it(single_xor(line, c as u8))))
            .max_by_key(|c| c.1).unwrap()))
        .max_by_key(|c| c.1.1).unwrap();

    println!("Solution is {:?} with score {:?}", result.1.0 , result.1.1);
    println!("Output is {:?}", single_xor(result.0, result.1.0 as u8));
}
