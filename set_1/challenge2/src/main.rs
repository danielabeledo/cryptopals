extern crate hex;

use hex::{from_hex, to_hex};
use std::env;

fn fixed_xor(input: &str, xor: &str) -> String {
    if input.len() != xor.len() {
        panic!("Input and xor strings need to be same length");
    }

    // xor'ing every byte
    let xor: Vec<u8> = from_hex(&input).into_iter()
        .zip(from_hex(&xor).into_iter())
        .map(|(a, b)| a ^ b)
        .collect();

    to_hex(xor)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let output = fixed_xor(&args[1], &args[2]);
    println!("{}", output);
}
