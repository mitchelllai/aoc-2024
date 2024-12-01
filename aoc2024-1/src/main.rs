use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    // Read file from AOC
    let mut file = File::open("/home/mmlai/aoc/aoc2024-1/input/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // String manipulation
    let lines = contents.split("\n");
    let mut left = vec![];
    let mut right = vec![];
    for line in lines {
        let split = line.split("   ").collect::<Vec<&str>>();
        left.push(split[0]);
        right.push(split[1]);
    }

    //Parse strings into ints and sort
    let mut left = left
        .iter()
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    left.sort();

    let mut right = right
        .iter()
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    right.sort();

    let solution1 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();

    println!("{solution1}");

    let mut right_count_map: HashMap<i32, i32> = HashMap::new();
    for right_num in right {
        *right_count_map.entry(right_num).or_insert(0) += 1;
    }

    let solution2 = left
        .iter()
        .map(|num| right_count_map.get(num).map_or(0, |count| count * num))
        .sum::<i32>();

    println!("{solution2}");
}
