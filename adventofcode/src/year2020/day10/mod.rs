use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let adapters: Vec<i32> = it.map(|s| s.parse::<i32>().unwrap()).collect();
    let res1 = part1(&adapters);
    let res2 = part2(&adapters);
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn part1(adapters: &[i32]) -> i32 {
    let mut v = vec![0];
    let mut max = 0;
    for &x in adapters {
        v.push(x);
        max = max.max(x);
    }
    v.push(max + 3);
    v.sort_unstable();
    let mut count = vec![0; 4];
    for w in v.windows(2) {
        let diff = w[1] - w[0];
        count[diff as usize] += 1;
    }
    count[1] * count[3]
}

fn part2(adapters: &[i32]) -> i64 {
    let mut v = vec![0];
    let mut max = 0;
    for &x in adapters {
        v.push(x);
        max = max.max(x);
    }
    v.push(max + 3);
    v.sort_unstable();
    let n = v.len();
    let mut dp = vec![0; n];
    dp[0] = 1;
    for i in 0..n {
        for j in (0..i).rev() {
            if v[i] - v[j] > 3 {
                break;
            }
            dp[i] += dp[j];
        }
    }
    dp[n - 1]
}

adventofcode!(test, "input.txt", "output.txt");
