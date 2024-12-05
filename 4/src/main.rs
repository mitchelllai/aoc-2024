use std::{fs::File, io::Read};
fn sol1() -> Result<i32, anyhow::Error> {
    // Read file from AOC
    let mut contents = String::new();
    File::open("./input")?.read_to_string(&mut contents)?;

    let grid = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut count = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, character) in row.iter().enumerate() {
            if *character == 'X' {
                let r_1 = j as isize + 1;
                let r_2 = j as isize + 2;
                let r_3 = j as isize + 3;
                let l_1 = j as isize - 1;
                let l_2 = j as isize - 2;
                let l_3 = j as isize - 3;
                let d_1 = i as isize + 1;
                let d_2 = i as isize + 2;
                let d_3 = i as isize + 3;
                let u_1 = i as isize - 1;
                let u_2 = i as isize - 2;
                let u_3 = i as isize - 3;
                if d_1 < grid.len() as isize
                    && d_2 < grid.len() as isize
                    && d_3 < grid.len() as isize
                {
                    let (m, a, s) = (
                        grid[d_1 as usize][j],
                        grid[d_2 as usize][j],
                        grid[d_3 as usize][j],
                    );
                    if m == 'M' && a == 'A' && s == 'S' {
                        count += 1;
                    }
                }
                if u_1 >= 0 && u_2 >= 0 && u_3 >= 0 {
                    let (m, a, s) = (
                        grid[u_1 as usize][j],
                        grid[u_2 as usize][j],
                        grid[u_3 as usize][j],
                    );
                    if m == 'M' && a == 'A' && s == 'S' {
                        count += 1;
                    }
                }
                if r_1 < row.len() as isize && r_2 < row.len() as isize && r_3 < row.len() as isize
                {
                    let (m, a, s) = (
                        grid[i][r_1 as usize],
                        grid[i][r_2 as usize],
                        grid[i][r_3 as usize],
                    );
                    if m == 'M' && a == 'A' && s == 'S' {
                        count += 1;
                    }
                }
                if l_1 >= 0 && l_2 >= 0 && l_3 >= 0 {
                    let (m, a, s) = (
                        grid[i][l_1 as usize],
                        grid[i][l_2 as usize],
                        grid[i][l_3 as usize],
                    );
                    if m == 'M' && a == 'A' && s == 'S' {
                        count += 1;
                    }
                }
                if d_1 < grid.len() as isize
                    && d_2 < grid.len() as isize
                    && d_3 < grid.len() as isize
                    && r_1 < row.len() as isize
                    && r_2 < row.len() as isize
                    && r_3 < row.len() as isize
                {
                    let (m, a, s) = (
                        grid[d_1 as usize][r_1 as usize],
                        grid[d_2 as usize][r_2 as usize],
                        grid[d_3 as usize][r_3 as usize],
                    );
                    if m == 'M' && a == 'A' && s == 'S' {
                        count += 1;
                    }
                }
                if d_1 < grid.len() as isize
                    && d_2 < grid.len() as isize
                    && d_3 < grid.len() as isize
                    && l_1 >= 0
                    && l_2 >= 0
                    && l_3 >= 0
                {
                    let (m, a, s) = (
                        grid[d_1 as usize][l_1 as usize],
                        grid[d_2 as usize][l_2 as usize],
                        grid[d_3 as usize][l_3 as usize],
                    );
                    if m == 'M' && a == 'A' && s == 'S' {
                        count += 1;
                    }
                }
                if u_1 >= 0
                    && u_2 >= 0
                    && u_3 >= 0
                    && r_1 < row.len() as isize
                    && r_2 < row.len() as isize
                    && r_3 < row.len() as isize
                {
                    let (m, a, s) = (
                        grid[u_1 as usize][r_1 as usize],
                        grid[u_2 as usize][r_2 as usize],
                        grid[u_3 as usize][r_3 as usize],
                    );
                    if m == 'M' && a == 'A' && s == 'S' {
                        count += 1;
                    }
                }
                if u_1 >= 0 && u_2 >= 0 && u_3 >= 0 && l_1 >= 0 && l_2 >= 0 && l_3 >= 0 {
                    let (m, a, s) = (
                        grid[u_1 as usize][l_1 as usize],
                        grid[u_2 as usize][l_2 as usize],
                        grid[u_3 as usize][l_3 as usize],
                    );
                    if m == 'M' && a == 'A' && s == 'S' {
                        count += 1;
                    }
                }
            }
        }
    }
    Ok(count)
}
fn sol2() -> Result<i32, anyhow::Error> {
    // Read file from AOC
    let mut contents = String::new();
    File::open("./input")?.read_to_string(&mut contents)?;

    let grid = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut count = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, character) in row.iter().enumerate() {
            if *character == 'A' {
                let r_1 = j as isize + 1;
                let l_1 = j as isize - 1;
                let d_1 = i as isize + 1;
                let u_1 = i as isize - 1;
                if d_1 < grid.len() as isize && r_1 < row.len() as isize && u_1 >= 0 && l_1 >= 0 {
                    let (c_1, c_2, c_3, c_4) = (
                        grid[u_1 as usize][l_1 as usize],
                        grid[d_1 as usize][l_1 as usize],
                        grid[u_1 as usize][r_1 as usize],
                        grid[d_1 as usize][r_1 as usize],
                    );
                    if c_1 == 'M' && c_2 == 'M' && c_3 == 'S' && c_4 == 'S'
                        || c_1 == 'M' && c_4 == 'S' && c_2 == 'S' && c_3 == 'M'
                        || c_2 == 'M' && c_3 == 'S' && c_1 == 'S' && c_4 == 'M'
                        || c_1 == 'S' && c_2 == 'S' && c_3 == 'M' && c_4 == 'M'
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    Ok(count)
}
fn main() -> Result<(), anyhow::Error> {
    println!("{}", sol1()?);
    println!("{}", sol2()?);
    Ok(())
}
