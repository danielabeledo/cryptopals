use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let block: &str = &args[1];
    let block_size: u8 = (&args[2]).parse().unwrap();

    // finding how much padding to add
    let remainder = (block.len() as u32 % block_size as u32) as u8;
    let padding: u8 = if remainder != 0 {block_size - remainder} else {block_size};
    let mut padded_block: Vec<u8> = Vec::new();
    padded_block.extend(block.as_bytes());
    for _ in 0..padding {
        padded_block.push(padding);
    }

    println!("{:?}", padded_block);
}
