extern crate hex;

use std::env;

fn fixed_xor(input: &str, xor: &str) -> String {
    if input.len() != xor.len() {
        panic!("Input and xor strings need to be same length");
    }
    let input_hex: Vec<u8> = hex::from_hex(&input);
    let input_xor: Vec<u8> = hex::from_hex(&xor);
    let mut result: Vec<u8> = Vec::with_capacity(input_hex.len());
    for (i, c) in input_hex.into_iter().enumerate() {
        result.push(c ^ input_xor[i]);
    }
    hex::to_hex(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let output = fixed_xor(&args[1], &args[2]);
    println!("{}", output);
}
