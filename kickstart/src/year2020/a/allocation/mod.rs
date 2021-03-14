use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) -> RustGymResult {
    let mut it = reader.lines().map(|l| l.unwrap());
    let t = it.next().unwrap().parse::<usize>()?;
    for i in 1..=t {
        let vals: Vec<i32> = it
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let n = vals[0] as usize;
        let b = vals[1];
        let mut houses: Vec<i32> = it
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        houses.sort_unstable();
        let mut sum = 0;
        let mut res = 0;
        for i in 0..n {
            if houses[i] + sum <= b {
                sum += houses[i];
                res += 1;
            }
        }
        writeln!(writer, "Case #{}: {}", i, res)?;
    }
    Ok(())
}

test_gen!(test, "input.txt", "output.txt");
