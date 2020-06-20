extern crate hex;

use std::env;

fn repeated_xor(input: &str, xor: &str) -> Vec<u8> {
    let xor_bytes = xor.bytes().collect::<Vec<u8>>();
    let mut output = Vec::with_capacity(input.len());
    for (i, b) in input.bytes().enumerate() {
        output.push(b ^ xor_bytes[i % xor_bytes.len()]);
    }
    output
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let output: Vec<u8> = repeated_xor(&args[1], &args[2]);
    println!("{:?}", hex::to_hex(output));
}
