use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    read_to_string("./src/8.txt").expect("file not found")
        .lines()
        .map(|line| (line, duplicated_blocks(line)))
        .filter(|(_, b)| *b)
        .for_each(|(line, _)| println!("{}", line));
}

fn duplicated_blocks(line: &str) -> bool {
    let mut set = HashSet::new();
    for chunk in hex::from_hex(line).chunks(16) {
        if !set.insert(chunk) {
            return true;
        }
    }
    false
}
