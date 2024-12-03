use std::{fs::File, io::Read};

fn is_safe_report(report: &Vec<i32>) -> bool {
    let mut increasing = false;
    let mut decreasing = false;
    let mut report = report.iter();
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
}

fn sol1() -> Result<(), anyhow::Error> {
    // Read file from AOC
    let mut contents = String::new();
    File::open("./input")?.read_to_string(&mut contents)?;

    // Count safe reports
    let safe_reports_count = contents
        .lines()
        .map(|line| line.split(' '))
        .map(|report_strs| {
            report_strs
                .map(|report_str| match report_str.parse::<i32>() {
                    Ok(num) => num,
                    Err(e) => panic!("{e}"),
                })
                .collect::<Vec<_>>()
        })
        .filter(is_safe_report)
        .collect::<Vec<_>>()
        .len();
    println!("{safe_reports_count}");
    Ok(())
}

fn sol2() -> Result<(), anyhow::Error> {
    // Read file from AOC
    let mut contents = String::new();
    File::open("./input")?.read_to_string(&mut contents)?;

    let original_reports = contents.lines().map(|line| {
        line.split(' ')
            .map(|num_str| match num_str.parse::<i32>() {
                Ok(num) => num,
                Err(e) => panic!("{e}"),
            })
            .collect::<Vec<_>>()
    });
    let mut reports = vec![];
    for original_report in original_reports {
        let mut report_group = vec![];
        report_group.push(original_report.clone());
        for i in 0..original_report.len() {
            let mut this_report = original_report.clone();
            this_report.remove(i);
            report_group.push(this_report);
        }
        reports.push(report_group);
    }
    let safe_reports_count = reports
        .iter()
        .filter(|report_group| {
            for report in report_group.iter() {
                if is_safe_report(report) {
                    return true;
                }
            }
            false
        })
        .collect::<Vec<_>>()
        .len();
    println!("{safe_reports_count}");

    Ok(())
}
fn main() -> Result<(), anyhow::Error> {
    sol1()?;
    sol2()?;
    Ok(())
}
