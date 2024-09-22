extern crate core;

use openssl::symm::{Cipher, Crypter, Mode};
use std::fs;

fn main() {
    // load data from file into a byte array
    let mut file_content: Vec<u8> = Vec::new();
    fs::read_to_string("./src/10.txt")
        .expect("file not found")
        .lines()
        .map(|line| line.as_bytes())
        .for_each(|line| line.iter().for_each(|b| file_content.push(*b)));

    // ciphertext
    let input: Vec<u8> = base64::from_base64(file_content.as_slice());

    // Key
    let key = "YELLOW SUBMARINE".as_bytes();

    // IV
    let iv: [u8; 16] = [0u8; 16];

    // implementing CBC
    let decrypted = decrypt_cbc(
        input,
        key,
        &iv,
    );

    println!("{}", String::from_utf8(decrypted).unwrap());
}
fn decrypt_cbc(input: Vec<u8>, key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    let blocks = input.chunks(16);
    let mut ctx = iv;

    // creating ecb decrypt no block crypter
    let mut crypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, key, None).unwrap();
    crypter.pad(false);

    for block in blocks {
        // AES ECB decrypt block - no pad
        let mut out = vec![0; 32];
        let count = crypter.update(block, &mut out).unwrap();
        let rest = crypter.finalize(&mut out[count..]).unwrap();
        out.truncate(count + rest);
        // xor'ing decrypted block with IV or previous encrypted block
        let xor = out.as_slice()
            .iter().zip(ctx.iter())
            .map(|(a, b)| a ^ b)
            .collect::<Vec<u8>>();
        // append to plain text
        result.append(&mut xor.to_vec());
        // updating ctx to be used in next XOR
        ctx = block;
    }

    unpad_result(result)
}

fn unpad_result(input: Vec<u8>) -> Vec<u8> {
    input[..(input.len() - input[input.len() - 1] as usize)].to_vec()
}

