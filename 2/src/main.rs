use std::{fs::File, io::Read};

fn sol1() -> Result<(), anyhow::Error> {
    // Read file from AOC
    let mut contents = String::new();
    File::open("./input")?.read_to_string(&mut contents)?;

    let safe_reports_count = contents
        .lines()
        .map(|line| line.split(' '))
        .map(|num_strs| num_strs.map(|num_str| num_str.parse::<i32>().unwrap()))
        .filter(|nums| {
            let mut increasing = false;
            let mut decreasing = false;
            let mut nums = nums.clone();
            let mut prev = nums.next();
            let mut curr = nums.next();
            while curr.is_some() {
                let prev_val = prev.unwrap();
                let curr_val = curr.unwrap();
                let diff = curr_val - prev_val;
                if diff == 0 {
                    return false;
                }
                if diff > 0 {
                    increasing = true;
                }
                if diff < 0 {
                    decreasing = true;
                }
                if increasing && decreasing {
                    return false;
                }
                if diff.abs() > 3 || diff.abs() < 1 {
                    return false;
                }
                prev = curr;
                curr = nums.next();
            }
            true
        })
        .collect::<Vec<_>>()
        .len();
    println!("{safe_reports_count}");
    Ok(())
}

fn sol2() -> Result<(), anyhow::Error> {
    // Read file from AOC
    let mut contents = String::new();
    File::open("./input")?.read_to_string(&mut contents)?;

    let mut reports = vec![];
    let original_reports = contents
        .lines()
        .map(|line| line.split(' '))
        .map(|num_strs| num_strs.map(|num_str| num_str.parse::<i32>().unwrap()));
    original_reports.for_each(|report| {
        let mut report_group = vec![];
        let report = report.collect::<Vec<i32>>();
        report_group.push(report.clone());
        for i in 0..report.len() {
            let mut this_report = report.clone();
            this_report.remove(i);
            report_group.push(this_report);
        }
        reports.push(report_group);
    });
    let safe_reports_count = reports
        .iter()
        .filter(|report_group| {
            !report_group
                .iter()
                .filter(|report| {
                    let mut report = report.iter();
                    let mut increasing = false;
                    let mut decreasing = false;
                    let mut prev = report.next();
                    let mut curr = report.next();
                    while curr.is_some() {
                        let prev_val = prev.unwrap();
                        let curr_val = curr.unwrap();
                        let diff = curr_val - prev_val;
                        if diff == 0 {
                            return false;
                        }
                        if diff > 0 {
                            increasing = true;
                        }
                        if diff < 0 {
                            decreasing = true;
                        }
                        if increasing && decreasing {
                            return false;
                        }
                        if diff.abs() > 3 || diff.abs() < 1 {
                            return false;
                        }
                        prev = curr;
                        curr = report.next();
                    }
                    true
                })
                .collect::<Vec<_>>()
                .is_empty()
        })
        .collect::<Vec<_>>()
        .len();
    println!("{safe_reports_count}");

    Ok(())
}
fn main() -> Result<(), anyhow::Error> {
    let _ = sol1();
    let _ = sol2();
    Ok(())
}
