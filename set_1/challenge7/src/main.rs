use openssl::symm::{decrypt, Cipher};
use std::fs;
use std::str::from_utf8;

fn main() {
    // load data from file into a byte array
    let mut file_content: Vec<u8> = Vec::new();
    fs::read_to_string("./src/7.txt")
        .expect("file not found")
        .lines()
        .map(|line| line.as_bytes())
        .for_each(|line| line.iter().for_each(|b| file_content.push(*b)));
    let input: Vec<u8> = base64::from_base64(file_content.as_slice());

    // secret key not very secret
    let key = "YELLOW SUBMARINE".as_bytes();

    // using openssl binding to decrypt ciphertext with provided key in ECB mode
    let plaintext = decrypt(Cipher::aes_128_ecb(), key, None, input.as_slice()).unwrap();

    println!("{}", from_utf8(&plaintext).unwrap());
}
