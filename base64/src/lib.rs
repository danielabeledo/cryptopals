use std::collections::HashMap;
use std::sync::OnceLock;

const DICTIONARY: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
fn to() -> &'static HashMap<u8, u8> {
    static HASHMAP: OnceLock<HashMap<u8, u8>> = OnceLock::new();
    HASHMAP.get_or_init(|| DICTIONARY.as_bytes().into_iter()
        .enumerate().map(|(i, &c)| (i as u8, c)).collect())
}
fn from() -> &'static HashMap<u8, u8> {
    static HASHMAP: OnceLock<HashMap<u8, u8>> = OnceLock::new();
    HASHMAP.get_or_init(|| DICTIONARY.as_bytes().into_iter()
        .enumerate().map(|(i, &c)| (c, i as u8)).collect())
}

fn get_to(value: u8) -> u8 {
    *to().get(&value).unwrap()
}

fn get_from(value: u8) -> u8 {
    if value == 0x3D {
        0b0
    } else {
        *from().get(&value).unwrap()
    }
}

// Gets a base64 encoded output for an array of bytes.
pub fn to_base64(input: Vec<u8>) -> Vec<u8> {
    if input.len() == 0 {
        panic!("input cannot be empty");
    };

    let mut result: Vec<u8> = Vec::with_capacity(input.len() * 4 / 3);
    // every chunk of 3 bytes, gets encoded into 4 characters from a 6-bit 64 characters dictionary
    // i.e. base64 encoding converts 3 bytes into 4 chars
    for chunk in input.chunks(3) {
        // first 6 bits is always in a chunk, regardless its size
        result.push(get_to(chunk[0] >> 2));

        if chunk.len() == 3 {
            // for the 24 bits chunk, we will add the 4 characters

            // takes 2 bits from first byte and 4 bits from second byte
            result.push(get_to(((chunk[0] & 0x03) << 4) | ((chunk[1] & 0xF0) >> 4)));
            // takes 4 bits from second byte and 2 bits from third byte
            result.push(get_to(((chunk[1] & 0x0F) << 2) | ((chunk[2] & 0xC0) >> 6)));
            // takes last 6 bites from third byte
            result.push(get_to(chunk[2] & 0x3F));
        } else if chunk.len() == 2 {
            // for a 16 bits chunk, we will add 3 chars and one padding

            // takes 2 bits from first byte and 4 bits from second byte
            result.push(get_to(((chunk[0] & 0x03) << 4) | ((chunk[1] & 0xF0) >> 4)));
            // takes 4 bits from the second byte with two unset bits in the lsb side
            result.push(get_to((chunk[1] & 0x0F) << 2));
            // adds a padding char
            result.push(0x3D);
        } else {
            // in a 8 bits chunk, we will only be able to take 2 bits from the first byte
            result.push(get_to((chunk[0] & 0x03) << 4));
            // and then add 2 padding chars
            result.push(0x3D);
            result.push(0x3D);
        }
    }
    result
}

// Gets a byte array from a base64 input
pub fn from_base64(input: &[u8]) -> Vec<u8> {
    if input.len() == 0 {
        panic!("input cannot be empty");
    };

    if input.len() % 4 != 0 {
        panic!("strict mode")
    }

    let mut result: Vec<u8> = Vec::new();
    for chunk in input.chunks(4) {
        // every chunk of 4 chars, gets decoded into 3 bytes
        // i.e. base64 decodes to 3 bytes from 4 chars
        result.push(get_from(chunk[0]) << 2 | (get_from(chunk[1]) & 0x30) >> 4);
        if chunk[2] != 0x3D || (get_from(chunk[1]) << 4) != 0 {
            result.push(get_from(chunk[1]) << 4 | (get_from(chunk[2]) & 0x3C) >> 2);
        }
        if chunk[3] != 0x3D || (get_from(chunk[2]) << 6) != 0 {
            result.push(get_from(chunk[2]) << 6 | get_from(chunk[3]));
        }
    }
    result.to_vec()
}

#[cfg(test)]
mod tests {
    use super::from_base64;
    use super::to_base64;
    use std::str;

    fn run_to(v: &str) -> String {
        String::from_utf8(to_base64(v.as_bytes().to_vec())).unwrap()
    }

    fn run_from(s: &str) -> String {
        let base64 = &from_base64(s.as_bytes());
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
