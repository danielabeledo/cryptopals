extern crate base64;
extern crate hex;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let from_hex: Vec<u8> = hex::from_hex(&args[1]);

    let base64_array: Vec<u8> = base64::to_base64(from_hex);

    println!("{}", String::from_utf8(base64_array).unwrap());
}
