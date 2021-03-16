use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) -> RustGymResult {
    let mut it = reader.lines().map(|l| l.unwrap());
    let t = it.next().unwrap().parse::<usize>()?;
    for i in 1..=t {
        let nums: Vec<f64> = it
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<f64>().unwrap())
            .collect();
        let res = degree(nums[0], nums[1]);
        writeln!(writer, "Case #{}: {:.7}", i, res)?;
    }
    Ok(())
}

fn degree(v: f64, d: f64) -> f64 {
    let mut left = 0.0;
    let mut right = 45.0;
    let target = d * 9.8 / (2.0 * v * v);
    loop {
        let mid: f64 = (left + right) / 2.0;
        let mid_r: f64 = mid.to_radians();
        let val = mid_r.sin() * mid_r.cos();
        let diff = (val - target).abs();
        if diff < std::f64::EPSILON {
            return mid;
        }
        if val > target {
            right = mid;
        }
        if val < target {
            left = mid;
        }
    }
}

test_gen!(test, "input.txt", "output.txt");
