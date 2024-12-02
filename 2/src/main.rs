use std::{collections::HashMap, fs::File, io::Read};
fn main() -> Result<(), anyhow::Error> {
    // Read file from AOC
    let mut contents = String::new();
    File::open("./input")?.read_to_string(&mut contents)?;

    for line in contents.lines() {
        let nums = line
            .split(' ')
            .map(|num_str| num_str.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        let nums_len = nums.len();
        let i = 0;
        let j = 1;
        while j < nums_len {}
    }

    println!("{contents}");

    Ok(())
}
