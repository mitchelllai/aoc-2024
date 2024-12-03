use std::{fs::File, io::Read};
fn sol1() -> Result<i32, anyhow::Error> {
    // Read file from AOC
    let mut contents = String::new();
    File::open("./input")?.read_to_string(&mut contents)?;

    let re = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)")?;
    let numbers = re.captures_iter(&contents).map(|capture| capture.extract());
    let mut total_sum = 0;
    for (_, [num0, num1]) in numbers {
        total_sum += num0.parse::<i32>()? * num1.parse::<i32>()?;
    }
    Ok(total_sum)
}

fn sol2() -> Result<i32, anyhow::Error> {
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
                    total_sum += num0.parse::<i32>()? * num1.parse::<i32>()?;
                }
            }
            _ => panic!("Should never reach here"),
        }
    }
    Ok(total_sum)
}
fn main() -> Result<(), anyhow::Error> {
    println!("{}", sol1()?);
    println!("{}", sol2()?);
    Ok(())
}
