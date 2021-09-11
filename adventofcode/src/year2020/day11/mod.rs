use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let grid: Vec<Vec<char>> = it.map(|s| s.chars().collect()).collect();
    let res1 = simulate1(grid.clone());
    let res2 = simulate2(grid);
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn simulate1(mut grid: Vec<Vec<char>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    loop {
        let mut occupied = 0;
        let mut change = 0;
        let mut next_grid = vec![vec!['.'; m]; n];
        for i in 0..n {
            for j in 0..m {
                let mut count = 0;
                for &di in &[-1, 0, 1] {
                    for &dj in &[-1, 0, 1] {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let ii = i as i32 + di;
                        let jj = j as i32 + dj;
                        if ii < 0 || ii >= n as i32 || jj < 0 || jj >= m as i32 {
                            continue;
                        }
                        if grid[ii as usize][jj as usize] == '#' {
                            count += 1;
                        }
                    }
                }
                next_grid[i][j] = grid[i][j];
                if count == 0 && grid[i][j] == 'L' {
                    next_grid[i][j] = '#';
                    change += 1;
                }
                if count >= 4 && grid[i][j] == '#' {
                    next_grid[i][j] = 'L';
                    change += 1;
                }
                if next_grid[i][j] == '#' {
                    occupied += 1;
                }
            }
        }
        if change == 0 {
            break occupied;
        }
        grid = next_grid;
    }
}

fn simulate2(mut grid: Vec<Vec<char>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    loop {
        let mut occupied = 0;
        let mut change = 0;
        let mut next_grid = vec![vec!['.'; m]; n];
        for i in 0..n {
            for j in 0..m {
                let mut count = 0;
                for &di in &[-1, 0, 1] {
                    for &dj in &[-1, 0, 1] {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let mut step = 1;
                        loop {
                            let ii = i as i32 + di * step;
                            let jj = j as i32 + dj * step;
                            if ii < 0 || ii >= n as i32 || jj < 0 || jj >= m as i32 {
                                break;
                            }

                            if grid[ii as usize][jj as usize] == '#' {
                                count += 1;
                                break;
                            }
                            if grid[ii as usize][jj as usize] == 'L' {
                                break;
                            }
                            step += 1;
                        }
                    }
                }
                next_grid[i][j] = grid[i][j];
                if count == 0 && grid[i][j] == 'L' {
                    next_grid[i][j] = '#';
                    change += 1;
                }
                if count >= 5 && grid[i][j] == '#' {
                    next_grid[i][j] = 'L';
                    change += 1;
                }
                if next_grid[i][j] == '#' {
                    occupied += 1;
                }
            }
        }
        if change == 0 {
            break occupied;
        }
        grid = next_grid;
    }
}

adventofcode!(test, "input.txt", "output.txt");
