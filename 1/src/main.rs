use std::{collections::HashMap, fs::File, io::Read};

fn main() -> Result<(), anyhow::Error> {
    // Read file from AOC
    let mut contents = String::new();
    File::open("./input")?.read_to_string(&mut contents)?;

    // Create left and right vectors as containers for two columns
    let mut left = vec![];
    let mut right = vec![];

    // String manipulation and parsing &str into i32
    for line in contents.lines() {
        let mut split = line.split("   ");
        if let Some(left_str) = split.next() {
            left.push(left_str.parse::<i32>()?);
        }
        if let Some(right_str) = split.next() {
            right.push(right_str.parse::<i32>()?);
        }
    }

    // Sort
    left.sort();
    right.sort();

    // Compute distance between left and right lists
    let solution1 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();
    println!("{solution1}");

    // Keep a running count of each item in right list
    let mut right_count_map = HashMap::new();
    right
        .iter()
        .for_each(|right_num| *right_count_map.entry(right_num).or_insert(0) += 1);

    // Compute similarity score
    let solution2 = left
        .iter()
        .map(|num| right_count_map.get(num).map_or(0, |count| count * num))
        .sum::<i32>();
    println!("{solution2}");

    Ok(())
}
