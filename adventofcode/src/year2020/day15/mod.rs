use rustgym_util::*;
use std::collections::HashMap;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let mut it = reader.lines().map(|l| l.unwrap());
    let nums: Vec<i32> = it
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let res1 = sequence(&nums, 2020);
    let res2 = sequence(&nums, 30000000);
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn sequence(nums: &[i32], end: usize) -> i32 {
    let n = nums.len();
    let mut hm: HashMap<i32, usize> = HashMap::new();
    let mut i = 0;
    let mut next = 0;
    while i < n {
        next = if let Some(j) = hm.insert(nums[i], i) {
            i - j
        } else {
            0
        } as i32;
        i += 1;
    }
    while i < end - 1 {
        next = if let Some(j) = hm.insert(next, i) {
            i - j
        } else {
            0
        } as i32;
        i += 1;
    }
    next
}

adventofcode_ignore!(test, "input.txt", "output.txt");
