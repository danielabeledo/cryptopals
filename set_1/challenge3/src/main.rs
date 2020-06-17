extern crate hex;

use std::collections::HashMap;
use std::env;
use std::str;

fn single_xor(input: &str, xor: u8) -> String {
    let input_hex: Vec<u8> = hex::from_hex(&input);
    let mut result: Vec<u8> = Vec::with_capacity(input_hex.len());
    for c in input_hex.into_iter() {
        result.push(c ^ xor);
    }
    str::from_utf8(&result).unwrap().to_owned()
}

fn get_freq_dictionary() -> HashMap<char, i16> {
    let mut result: HashMap<char, i16> = HashMap::with_capacity(13);
    for (c, i) in [
        (' ', 8),
        ('e', 5),
        ('t', 3),
        ('a', 3),
        ('o', 3),
        ('i', 3),
        ('n', 3),
        ('s', 2),
        ('r', 2),
        ('h', 2),
        ('l', 1),
        ('d', 1),
        ('u', 1),
    ]
    .iter_mut()
    {
        result.insert(*c, *i);
    }
    result
}

fn score_it(frequencies: &HashMap<char, i16>, input: String) -> i16 {
    let mut score: i16 = 0;
    for c in input.chars() {
        score = score + frequencies.get(&c).unwrap_or(&0);
    }
    score
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let frequencies = get_freq_dictionary();
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars();
    let mut best_score: i16 = 0;
    let mut solution = ' ';
    for c in chars {
        let result = single_xor(&args[1], c as u8);
        let score = score_it(&frequencies, result);
        if score >= best_score {
            best_score = score;
            solution = c;
        }
    }
    println!("Solution is {:?} with score {:?}", solution, best_score);
    println!("Output is {:?}", single_xor(&args[1], solution as u8));
}
