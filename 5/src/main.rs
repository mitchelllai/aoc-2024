use std::{collections::HashSet, fs::File, io::Read};

fn main() -> Result<(), anyhow::Error> {
    let mut rules_raw = String::new();
    File::open("./rules")?.read_to_string(&mut rules_raw)?;
    let mut rules = HashSet::<(i32, i32)>::new();
    rules_raw
        .lines()
        .map(|rule| rule.split("|"))
        .map(|mut nums| (nums.next(), nums.next()))
        .map(|(num_0_str, num_1_str)| {
            (
                num_0_str.unwrap().parse::<i32>().unwrap(),
                num_1_str.unwrap().parse::<i32>().unwrap(),
            )
        })
        .for_each(|rule| {
            rules.insert(rule);
        });

    let mut updates_raw = String::new();
    File::open("./updates")?.read_to_string(&mut updates_raw)?;
    let valid_updates_sum = updates_raw
        .lines()
        .map(|update| update.split(","))
        .map(|update| {
            update
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|update| {
            let update_len = update.len();
            for i in 0..update_len - 1 {
                for j in i + 1..update_len {
                    let this_rule = (update[i], update[j]);
                    if rules.get(&this_rule).is_none() {
                        return false;
                    }
                }
            }
            return true;
        })
        .map(|update| update[update.len() / 2])
        .sum::<i32>();
    println!("{:?}", valid_updates_sum);

    let invalid_updates_sum = updates_raw
        .lines()
        .map(|update| update.split(","))
        .map(|update| {
            update
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|update| {
            let update_len = update.len();
            for i in 0..update_len - 1 {
                for j in i + 1..update_len {
                    let this_rule = (update[i], update[j]);
                    if rules.get(&this_rule).is_none() {
                        return true;
                    }
                }
            }
            return false;
        })
        .map(|mut update| {
            let update_len = update.len();
            for i in 0..update_len - 1 {
                for j in i + 1..update_len {
                    let this_rule = (update[i], update[j]);
                    if rules.get(&this_rule).is_none() {
                        let temp = update[j];
                        update[j] = update[i];
                        update[i] = temp;
                    }
                }
            }
            update
        })
        .map(|update| update[update.len() / 2])
        .sum::<i32>();
    println!("{:?}", invalid_updates_sum);
    Ok(())
}
