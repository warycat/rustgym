use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

fn solve(case_no: usize, reader: &mut impl BufRead, writer: &mut impl Write) {
    let args: Vec<f64> = reader.parse_vec();
    let v = args[0];
    let d = args[1];
    let res = degree(v, d);
    writeln!(writer, "Case #{}: {:.7}", case_no, res).unwrap();
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

google_test_gen!(test, "input.txt", "output.txt");
