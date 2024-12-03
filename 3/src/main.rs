use std::{fs::File, io::Read};
fn sol1() -> Result<(), anyhow::Error> {
    // Read file from AOC
    let mut contents = String::new();
    File::open("./input")?.read_to_string(&mut contents)?;

    let re = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)")?;
    let total_sum = re
        .captures_iter(&contents)
        .map(|capture| capture.extract())
        .map(|(_, [num0, num1])| num0.parse::<i32>().unwrap() * num1.parse::<i32>().unwrap())
        .sum::<i32>();
    println!("{total_sum}");
    Ok(())
}

fn sol2() -> Result<(), anyhow::Error> {
    // Read file from AOC
    let mut contents = String::new();
    File::open("./input")?.read_to_string(&mut contents)?;

    let mut total_sum = 0;
    let re = regex::Regex::new(r"(mul)\(([0-9]+),([0-9]+)\)|(do\(\))()()|(don\'t\(\))()()")?;
    let captures = re.captures_iter(&contents);
    let mut mul_enabled = true;
    for capture in captures {
        let (_, [instruction, num0, num1]) = capture.extract();
        match instruction {
            "do()" => mul_enabled = true,
            "don't()" => mul_enabled = false,
            "mul" => {
                if mul_enabled {
                    total_sum += num0.parse::<i32>().unwrap() * num1.parse::<i32>().unwrap();
                }
            }
            _ => panic!("Should never reach here"),
        }
    }
    println!("{total_sum}");
    Ok(())
}
fn main() -> Result<(), anyhow::Error> {
    sol1()?;
    sol2()?;
    Ok(())
}
