use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let block: &str = &args[1];
    let block_size: u32 = (&args[2]).parse().unwrap();

    // finding how much padding to add
    let remainder = block.len() as u32 % block_size;
    let padding: u32 = if remainder != 0 {block_size - remainder} else {0};

    println!("{:?}", format!("{}{}", block, str::repeat("\x04", padding as usize)))
}
