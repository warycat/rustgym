use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let nums: Vec<i64> = it.map(|s| s.parse::<i64>().unwrap()).collect();
    let size = 25;
    let res1 = invalid_number(&nums, size);
    let res2 = encryption_weakness(&nums, res1);
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn invalid_number(nums: &[i64], size: usize) -> i64 {
    let n = nums.len();
    for i in size..n {
        if !valid(nums, i, size) {
            return nums[i];
        }
    }
    panic!()
}

fn valid(nums: &[i64], i: usize, size: usize) -> bool {
    for j in i - size..i {
        for k in j + 1..i {
            if nums[j] + nums[k] == nums[i] {
                return true;
            }
        }
    }
    false
}

fn encryption_weakness(nums: &[i64], sum: i64) -> i64 {
    let mut start = 0;
    let mut end = 0;
    let mut cur = 0;
    while cur != sum {
        if cur < sum {
            cur += nums[end];
            end += 1;
            continue;
        }
        if cur > sum {
            cur -= nums[start];
            start += 1;
            continue;
        }
    }
    let max = nums[start..end].iter().max().unwrap();
    let min = nums[start..end].iter().min().unwrap();
    max + min
}

adventofcode_ignore!(test, "input.txt", "output.txt");
