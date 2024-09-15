use std::collections::HashMap;
use std::fs;
use std::iter::Iterator;
use std::ops::RangeInclusive;
use std::str::from_utf8;
use std::sync::OnceLock;
const KEY_SIZE_RANGE: RangeInclusive<usize> = 5..=40;

fn main() {
    // load data from file into a byte array
    let mut file_content: Vec<u8> = Vec::new();
    fs::read_to_string("./src/6.txt")
        .expect("file not found")
        .lines()
        .map(|line| line.as_bytes())
        .for_each(|line| line.iter().for_each(|b| file_content.push(*b)));

    let input: Vec<u8> = base64::from_base64(file_content.as_slice());

    // using hamming distance, find key size
    let (key_size, score) = calculate_key_size(&input);
    println!("Key size is {} with score {}", key_size, score);

    // transpose blocks
    let mut original: Vec<Vec<u8>> = vec![vec![0; key_size]; input.len() / key_size + 1];
    for (i, b) in input.iter().enumerate() {
        original[i / key_size][i % key_size] = *b;
    }
    let transposed = transpose(original);

    // find single key xor for each transposed block
    let mut xor_key: Vec<u8> = Vec::with_capacity(key_size);
    for vector in transposed {
        // loop through all potential character-solutions and score the solutions based on the
        // frequency histogram - the one with the max value is the result.
        xor_key.push(get_vignere_key_char(&vector));
    }
    println!("XOR key is {:?}", from_utf8(&xor_key).unwrap());

    let mut output = Vec::with_capacity(input.len());
    for (i, b) in input.iter().enumerate() {
        output.push(b ^ xor_key[i % key_size]);
    }
    println!("{}", from_utf8(&output).unwrap());
}

///
/// Transposing a matrix (vec of vecs)
///
fn transpose(v: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect()
}

///
/// Gets a single Char for a Vigenère XOR key
///
fn get_vignere_key_char(vector: &Vec<u8>) -> u8 {
    (0..128_u8).into_iter()
        .map(|c| (c, vector.iter().map(|b| *b ^ c)
            .map(|c| get_freq_dictionary().get(&c).unwrap_or(&0))
            .sum::<u32>()))
        .max_by(|(_, c1), (_, c2)| c1.partial_cmp(&c2).unwrap())
        .unwrap().0
}
///
/// Calculating a probable Key Size for our Vigenère XOR key
///
fn calculate_key_size(input: &Vec<u8>) -> (usize, f32) {
    KEY_SIZE_RANGE
        .map(|i| (i, calculate(&input, i)))
        .min_by(|(_, c1), (_, c2)| c1.partial_cmp(&c2).unwrap())
        .unwrap()
}

///
/// Calculate average Hamming distance of a collection of
/// byte arrays with size {key_size}.
///
fn calculate(input: &Vec<u8>, key_size: usize) -> f32 {
    let groups: usize = input.len() / key_size;

    (0..groups - 1)
        .map(|i| hamming_distance_bits(
            &input[(i * key_size)..(i + 1) * key_size],
            &input[(i + 1) * key_size..(i + 2) * key_size],
        )).sum::<u32>() as f32 / key_size as f32 / groups as f32
}

///
/// Calculating Hamming distance between two arrays of same length
/// https://en.wikipedia.org/wiki/Hamming_distance
///
fn hamming_distance_bits(input1: &[u8], input2: &[u8]) -> u32 {
    if input1.len() != input2.len() {
        panic!("Input lengths don't match.")
    }
    input1.iter()
        .zip(input2)
        .fold(0, |a, (b, c)| a + (*b ^ *c).count_ones())
}

///
/// This is a normalised frequency dictionary of some of the most used
/// characters in the english language, starting with the space character.
/// The values are normalised, meaning that space will appear aprox. 8 times
/// more often than the letter u.
///
fn get_freq_dictionary() -> &'static HashMap<u8, u32> {
    static HASHMAP: OnceLock<HashMap<u8, u32>> = OnceLock::new();
    HASHMAP.get_or_init(||
        fs::read_to_string("src/frequency.txt").expect("file doesn't exist")
            .lines()
            .map(|line| line.as_bytes())
            .map(|line| (line[0], from_utf8(&line[2..])
                .map(|s| s.parse::<u32>().unwrap())
                .unwrap()))
            .collect()
    )
}

#[cfg(test)]
mod tests {
    use crate::hamming_distance_bits;

    #[test]
    fn test_hamming_distance_bits() {
        assert_eq!(37, hamming_distance_bits("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()));
    }
}