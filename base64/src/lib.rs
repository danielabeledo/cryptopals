use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DICTIONARY_TO: HashMap<u8, char> =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
            .chars()
            .enumerate()
            .map(|(i, c)| (i as u8, c))
            .collect();
    static ref DICTIONARY_FROM: HashMap<char, u8> =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
            .chars()
            .enumerate()
            .map(|(i, c)| (c, i as u8))
            .collect();
}

fn get_to(value: u8) -> char {
    *DICTIONARY_TO.get(&value).unwrap()
}

fn get_from(value: char) -> u8 {
    if value == '=' {
        0b0
    } else {
        *DICTIONARY_FROM.get(&value).unwrap()
    }
}

pub fn to_base64(input: Vec<u8>) -> Vec<char> {
    if input.len() == 0 {
        panic!("input cannot be empty");
    };

    let mut result: Vec<char> = Vec::with_capacity(input.len() * 4 / 3);
    for chunk in input.chunks(3) {
        if chunk.len() == 3 {
            result.push(get_to(chunk[0] >> 2));
            result.push(get_to(((chunk[0] & 0x03) << 4) | ((chunk[1] & 0xF0) >> 4)));
            result.push(get_to(((chunk[1] & 0x0F) << 2) | ((chunk[2] & 0xC0) >> 6)));
            result.push(get_to(chunk[2] & 0x3F));
        } else if chunk.len() == 2 {
            result.push(get_to(chunk[0] >> 2));
            result.push(get_to(((chunk[0] & 0x03) << 4) | ((chunk[1] & 0xF0) >> 4)));
            result.push(get_to((chunk[1] & 0x0F) << 2));
            result.push('=');
        } else {
            result.push(get_to(chunk[0] >> 2));
            result.push(get_to((chunk[0] & 0x03) << 4));
            result.push('=');
            result.push('=');
        }
    }
    result
}

pub fn from_base64(input: Vec<char>) -> Vec<u8> {
    if input.len() == 0 {
        panic!("input cannot be empty");
    };

    let mut result: Vec<u8> = Vec::new();
    for chunk in input.chunks(4) {
        if chunk.len() == 4 {
            result.push(get_from(chunk[0]) << 2 | (get_from(chunk[1]) & 0b110000) >> 4);
            result.push(get_from(chunk[1]) << 4 | (get_from(chunk[2]) & 0b111100) >> 2);
            result.push(get_from(chunk[2]) << 6 | get_from(chunk[3]));
        } else if chunk.len() == 3 {
            result.push(get_from(chunk[0]) << 2 | (get_from(chunk[1]) & 0b110000) >> 4);
            result.push(get_from(chunk[1]) << 4 | (get_from(chunk[2]) & 0b111100) >> 2);
            result.push(get_from(chunk[2]) << 6);
        } else if chunk.len() == 2 {
            result.push(get_from(chunk[0]) << 2 | (get_from(chunk[1]) & 0b110000) >> 4);
            result.push(get_from(chunk[1]) << 4);
        } else {
            result.push(get_from(chunk[0]) << 2);
        }
    }
    result.into_iter().filter(|&i| i != 0b0).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::from_base64;
    use super::to_base64;
    use std::iter::FromIterator;
    use std::str;

    fn run_to(v: &str) -> String {
        String::from_iter(to_base64(v.to_string().into_bytes()))
    }

    fn run_from(s: &str) -> String {
        let base64 = &from_base64(s.chars().collect());
        match str::from_utf8(base64) {
            Ok(value) => value.to_string(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        }
    }

    #[test]
    fn to_base64_test1() {
        assert_eq!("aW5wdXQ=", run_to("input"));
    }

    #[test]
    fn to_base64_test2() {
        assert_eq!("d29yZA==", run_to("word"));
    }

    #[test]
    fn to_base64_test3() {
        assert_eq!("YXdlc29tZSB0ZXN0", run_to("awesome test"));
    }

    #[test]
    #[should_panic(expected = "input cannot be empty")]
    fn to_base64_panic() {
        run_to("");
    }

    #[test]
    fn from_base64_test1() {
        assert_eq!("input", run_from("aW5wdXQ="));
    }

    #[test]
    fn from_base64_test2() {
        assert_eq!("word", run_from("d29yZA=="));
    }

    #[test]
    fn from_base64_test3() {
        assert_eq!("awesome test", run_from("YXdlc29tZSB0ZXN0"));
    }

    #[test]
    #[should_panic(expected = "input cannot be empty")]
    fn from_base64_panic() {
        run_from("");
    }
}
