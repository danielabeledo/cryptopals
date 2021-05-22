extern crate base64;

use std::collections::HashMap;
use std::fs;
use std::str;

fn hamming_distance_bits(input1: &[u8], input2: &[u8]) -> u32 {
    let mut result: u32 = 0;
    if input1.len() != input2.len() {
        panic!("Input lengths don't match.")
    }
    for i in 0..input1.len() {
        for j in 0..8 {
            if input1[i] & (1 << j) != input2[i] & (1 << j) {
                result = result + 1;
            }
        }
    }
    result
}

fn calculate(input: &Vec<u8>, keysize: usize) -> f32 {
    let mut rs: u32 = 0;
    let groups = input.len() / keysize - 1;
    for i in 0..groups {
        let first = &input[i * keysize..(i + 1) * keysize];
        let second = &input[(i + 1) * keysize..(i + 2) * keysize];
        rs = rs + hamming_distance_bits(first, second)
    }

    rs as f32 / keysize as f32 / groups as f32
}

fn calculate_range(input: &Vec<u8>) -> (usize, f32) {
    let mut best_value: f32 = 100.0;
    let mut best_key_size: usize = 0;
    for i in 2..40 {
        let value: f32 = calculate(&input, i);
        println!("{:?} {:?}", i, value);
        if value < best_value {
            best_value = value;
            best_key_size = i;
        }
    }
    (best_key_size, best_value)
}

fn main() {
    // load data from file into a byte array
    let input: Vec<u8> = {
        let mut vec: Vec<u8> = Vec::new();
        for line in fs::read_to_string("./src/6.txt")
            .expect("file not found")
            .lines()
        {
            vec.append(&mut base64::from_base64(
                line.trim().chars().collect::<Vec<char>>(),
            ));
        }
        vec
    };
    // using hamming distance, find key size
    let (key_size, score) = calculate_range(&input);
    println!("Keysize is {:?} with score {:?}", key_size, score);
    let key_size = 29;
    // transpose blocks
    let mut array: Vec<Vec<u8>> = vec![vec![0; input.len() / key_size + 1]; key_size];
    for (i, b) in input.iter().enumerate() {
        array[i % key_size][i / key_size] = *b;
    }

    let frequencies = get_freq_dictionary();
    // find single key xor for each transposed block
    let mut xor_key: Vec<u8> = Vec::with_capacity(key_size);
    for vector in array {
        let mut best_score: i16 = 0;
        let mut solution = 0;
        for c in 0..255 {
            let result: Vec<u8> = single_xor(&vector, c as u8);
            let score = score_it(&frequencies, result);
            if score >= best_score {
                best_score = score;
                solution = c;
            }
        }
        xor_key.push(solution);
    }
    println!("{:?}", str::from_utf8(&xor_key));

    let mut output = Vec::with_capacity(input.len());
    for (i, b) in input.iter().enumerate() {
        output.push(b ^ xor_key[i % key_size]);
    }
    println!("{:?}", str::from_utf8(output.as_slice()));

    let value = hamming_distance_bits("this is a test".as_bytes(), "wokka wokka!!!".as_bytes());
    println!("{:?}", value);
}

fn single_xor(input: &Vec<u8>, xor: u8) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(input.len());
    for c in input.into_iter() {
        result.push(c ^ xor);
    }
    result
}

fn get_freq_dictionary() -> HashMap<u8, i16> {
    let mut result: HashMap<u8, i16> = HashMap::with_capacity(13);
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
        result.insert(*c as u8, *i);
    }
    result
}

fn score_it(frequencies: &HashMap<u8, i16>, input: Vec<u8>) -> i16 {
    let mut score: i16 = 0;
    for c in input {
        score = score + frequencies.get(&c).unwrap_or(&0);
    }
    score
}
