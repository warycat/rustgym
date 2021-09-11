use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let mut grid: Vec<Vec<char>> = vec![];
    for line in it {
        grid.push(line.chars().collect());
    }

    let res1 = number_of_trees(&grid, 1, 1);
    let res2 = number_of_trees(&grid, 3, 1);
    let res3 = number_of_trees(&grid, 5, 1);
    let res4 = number_of_trees(&grid, 7, 1);
    let res5 = number_of_trees(&grid, 1, 2);
    writeln!(writer, "{}", res2).unwrap();
    writeln!(writer, "{}", res1 * res2 * res3 * res4 * res5).unwrap();
}

fn number_of_trees(grid: &[Vec<char>], right: usize, down: usize) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut i = 0;
    let mut j = 0;
    let mut res = 0;
    while i < n {
        if grid[i][j] == '#' {
            res += 1;
        }
        j += right;
        j %= m;
        i += down;
    }
    res
}

adventofcode!(test, "input.txt", "output.txt");
