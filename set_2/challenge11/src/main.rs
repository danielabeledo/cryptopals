use openssl::symm::{encrypt, Cipher};
use rand::{thread_rng, Rng, RngCore};
use std::collections::HashSet;

fn main() {
    let input = "A".repeat(128).into_bytes();

    let ciphertext = encryption_oracle(&input);

    println!("{:?}", ciphertext);

    if test_ecb(&ciphertext) == true {
        println!("ECB detected")
    } else {
        println!("must be CBC")
    }
}

fn generate_random_key(length: usize) -> Vec<u8> {
    let mut key: Vec<u8> = vec![0; length];
    thread_rng().fill_bytes(&mut key);
    key
}

fn encryption_oracle(input: &[u8]) -> Vec<u8> {
    let key = generate_random_key(16);
    let mut to_encrypt: Vec<u8> = Vec::new();

    // adding 5-10 random bytes
    (0..thread_rng().gen_range(5..=10))
        .for_each(|i| to_encrypt.push(i));
    // adding plain text
    to_encrypt.append(input.to_vec().as_mut());
    // adding another 5-10 random bytes
    (0..thread_rng().gen_range(5..=10))
        .for_each(|i| to_encrypt.push(i));

    // padding
    let remainder = (to_encrypt.len() as u32 % 16u32) as u8;
    let padding: u8 = if remainder != 0 { 16 - remainder } else { 16 };
    for _ in 0..padding {
        to_encrypt.push(padding);
    }

    if thread_rng().gen_bool(0.5) {
        println!("Using CBC");
        let mut iv: Vec<u8> = vec![0; 16];
        thread_rng().fill_bytes(&mut iv);
        encrypt(Cipher::aes_128_cbc(), &key, Some(&iv), input).unwrap()
    } else {
        println!("Using ECB");
        encrypt(Cipher::aes_128_ecb(), &key, None, input).unwrap()
    }
}

fn test_ecb(ciphertext: &[u8]) -> bool {
    let mut set = HashSet::new();
    for chunk in ciphertext.chunks(16) {
        if !set.insert(chunk) {
            return true;
        }
    }
    false
}
