use std::collections::HashMap;
use std::str;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DICTIONARY_TO: HashMap<u8, char> = "0123456789abcdef"
        .chars()
        .enumerate()
        .map(|(i, c)| (i as u8, c))
        .collect();
    static ref DICTIONARY_FROM: HashMap<char, u8> = "0123456789abcdef"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i as u8))
        .collect();
}

fn get_from(v1: char, v2: char) -> u8 {
    *DICTIONARY_FROM.get(&v1).unwrap() << 4 | *DICTIONARY_FROM.get(&v2).unwrap() & 0b1111
}

fn get_to(c: u8) -> char {
    *DICTIONARY_TO.get(&c).unwrap()
}

pub fn from_hex(input: &str) -> Vec<u8> {
    if input.len() % 2 != 0 {
        panic!("invalid hex")
    }
    let mut result: Vec<u8> = Vec::with_capacity(input.len() / 2);
    let chars = input.chars().collect::<Vec<char>>();
    for i in (0..chars.len()).step_by(2) {
        result.push(get_from(chars[i], chars[i + 1]));
    }
    result
}

pub fn to_hex(input: Vec<u8>) -> String {
    let mut result: Vec<char> = Vec::with_capacity(input.len() * 2);
    for c in input.into_iter() {
        result.push(get_to(c >> 4));
        result.push(get_to(c & 0x0F));
    }
    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::from_hex;
    use super::to_hex;

    #[test]
    fn from_hex_test1() {
        assert_eq!([073, 039, 109, 032, 107, 105, 108, 108, 105, 110, 103, 032, 121, 111, 117, 114, 032, 098, 114, 097, 105,
             110, 032, 108, 105, 107, 101, 032, 097, 032, 112, 111, 105, 115, 111, 110, 111, 117, 115, 032, 109, 117, 115, 104, 114, 111, 111, 109].to_vec(), from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
    }

    #[test]
    fn from_hex_test2() {
        assert_eq!([171, 60, 62].to_vec(), from_hex("ab3c3e"));
    }

    #[test]
    fn to_hex_test1() {
        assert_eq!("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
        to_hex([073, 039, 109, 032, 107, 105, 108, 108, 105, 110, 103, 032, 121, 111, 117, 114, 032, 098, 114, 097, 105, 110, 032, 108, 105, 107, 101, 032, 097, 032, 112, 111, 105, 115, 111, 110,  111, 117, 115, 032, 109, 117, 115, 104, 114, 111, 111, 109].to_vec()));
    }

    #[test]
    fn to_hex_test2() {
        assert_eq!("ab3c3e", to_hex([171, 60, 62].to_vec()));
    }
}
