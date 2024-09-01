extern crate hex;

use fs::read_to_string;
use std::{env, fs};
use hex::to_hex;

fn repeated_xor(input: &str, xor: &str) -> Vec<u8> {
    let xor_bytes = xor.as_bytes();
    input.as_bytes().into_iter().enumerate().map(|(i, b)| b ^ xor_bytes[i % xor_bytes.len()]).collect()
}

fn main() {
    // getting XOR word
    let args: Vec<String> = env::args().collect();

    // getting text from file
    let text = read_to_string("./src/5.txt").expect("file not found");

    // print solution
    println!("{}", to_hex(repeated_xor(&text, &args[1])));
}
