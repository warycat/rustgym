use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let mut res1 = 0;
    let mut res2 = 0;
    for line in it {
        let sides: Vec<i32> = line.split('x').map(|s| s.parse::<i32>().unwrap()).collect();
        res1 += paper(&sides);
        res2 += ribbon(&sides);
    }
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn paper(sides: &[i32]) -> i32 {
    let mut min = std::i32::MAX;
    let mut sum = 0;
    for i in 0..3 {
        let a = sides[i];
        let b = sides[(i + 1) % 3];
        min = min.min(a * b);
        sum += a * b * 2;
    }
    sum + min
}

fn ribbon(sides: &[i32]) -> i32 {
    let mut min = std::i32::MAX;
    let mut product = 1;
    for i in 0..3 {
        let a = sides[i];
        let b = sides[(i + 1) % 3];
        min = min.min(a + b);
        product *= a;
    }
    product + min * 2
}

adventofcode!(test, "input.txt", "output.txt");
