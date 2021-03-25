use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

fn solve(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = reader.parse_line();
    let c: Vec<i32> = reader.parse_vec();
    let res = jumping_on_the_clouds(n, c);
    write!(writer, "{}", res).unwrap();
}

fn jumping_on_the_clouds(n: usize, c: Vec<i32>) -> usize {
    let mut dp = vec![std::usize::MAX; n];
    dp[0] = 0;
    for i in 1..n {
        if c[i] == 1 {
            continue;
        }
        if i > 0 && c[i - 1] == 0 {
            dp[i] = dp[i].min(dp[i - 1] + 1);
        }
        if i > 1 && c[i - 2] == 0 {
            dp[i] = dp[i].min(dp[i - 2] + 1);
        }
    }
    dp[n - 1]
}

test_gen!(test00, "input00.txt", "output00.txt");
test_gen!(test01, "input01.txt", "output01.txt");
