extern crate base64;
extern crate hex;

use std::env;
use std::iter::FromIterator;

fn main() {
    let args: Vec<String> = env::args().collect();

    let from_hex: Vec<u8> = hex::from_hex(&args[1]);

    let base64_array: Vec<char> = base64::to_base64(from_hex);

    println!("{}", String::from_iter(base64_array));
}
