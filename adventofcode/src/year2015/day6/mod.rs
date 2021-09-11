use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let mut grid1: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];
    let mut grid2: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];
    for line in it {
        let words: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        let data = if words[1] == "on" || words[1] == "off" {
            (2, 4, if words[1] == "on" { 1 } else { 0 })
        } else {
            (1, 3, 2)
        };
        let cord1: Vec<usize> = words[data.0]
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let cord2: Vec<usize> = words[data.1]
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        light1(&mut grid1, data.2, &cord1, &cord2);
        light2(&mut grid2, data.2, &cord1, &cord2);
    }
    let res1: i32 = grid1.iter().map(|r| r.iter().sum::<i32>()).sum();
    let res2: i32 = grid2.iter().map(|r| r.iter().sum::<i32>()).sum();
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn light1(grid: &mut Vec<Vec<i32>>, cmd: i32, cord1: &[usize], cord2: &[usize]) {
    let r1 = cord1[0].min(cord2[0]);
    let c1 = cord1[1].min(cord2[1]);
    let r2 = cord1[0].max(cord2[0]);
    let c2 = cord1[1].max(cord2[1]);
    for i in r1..=r2 {
        for j in c1..=c2 {
            if cmd == 2 {
                grid[i][j] = 1 - grid[i][j];
            } else {
                grid[i][j] = cmd;
            }
        }
    }
}

fn light2(grid: &mut Vec<Vec<i32>>, cmd: i32, cord1: &[usize], cord2: &[usize]) {
    let r1 = cord1[0].min(cord2[0]);
    let c1 = cord1[1].min(cord2[1]);
    let r2 = cord1[0].max(cord2[0]);
    let c2 = cord1[1].max(cord2[1]);
    for i in r1..=r2 {
        for j in c1..=c2 {
            if cmd == 0 {
                grid[i][j] -= 1;
                grid[i][j] = grid[i][j].max(0);
            } else {
                grid[i][j] += cmd;
            }
        }
    }
}

adventofcode!(test, "input.txt", "output.txt");
