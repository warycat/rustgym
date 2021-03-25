use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

fn solve(case_no: usize, reader: &mut impl BufRead, writer: &mut impl Write) {
    let args: Vec<f64> = reader.parse_vec();
    let f = args[0];
    let r0 = args[1];
    let t = args[2];
    let r = args[3];
    let g = args[4];
    let res = probability(f, r0, t, r, g);
    writeln!(writer, "Case #{}: {:.6}", case_no, res).unwrap();
}

fn probability(f: f64, r0: f64, t: f64, r: f64, g: f64) -> f64 {
    let mut area = 0.0;
    let r1 = r0 - t - f;
    let r2 = r1 * r1;
    let mut x1 = r + f;
    while x1 < r1 {
        let mut y1 = r + f;
        while y1 < r1 {
            let x2 = x1 + g - 2.0 * f;
            let y2 = y1 + g - 2.0 * f;
            area += square(x1, x2, y1, y2, r1, r2);
            y1 += g + 2.0 * r;
        }
        x1 += g + 2.0 * r;
    }
    1.0 - area / (r0 * r0 * std::f64::consts::PI / 4.0)
}

fn square(x1: f64, x2: f64, y1: f64, y2: f64, r1: f64, r2: f64) -> f64 {
    if x2 <= x1 || y2 <= y1 || x1 * x1 + y1 * y1 >= r2 {
        0.0
    } else {
        let c = (x2 - x1) * (y2 - y1);
        if x2 * x2 + y2 * y2 <= r2 {
            c
        } else if x1 * x1 + y2 * y2 >= r2 && x2 * x2 + y1 * y1 >= r2 {
            let a = circle_segment(r1, (x1 / r1).acos() - (y1 / r1).asin());
            let b = ((r2 - x1 * x1).sqrt() - y1) * ((r2 - y1 * y1).sqrt() - x1) / 2.0;
            a + b
        } else if x1 * x1 + y2 * y2 >= r2 {
            let a = circle_segment(r1, (x1 / r1).acos() - (x2 / r1).acos());
            let b = ((r2 - x1 * x1).sqrt() - y1 + (r2 - x2 * x2).sqrt() - y1) * (x2 - x1) / 2.0;
            a + b
        } else if x2 * x2 + y1 * y1 >= r2 {
            let a = circle_segment(r1, (y2 / r1).asin() - (y1 / r1).asin());
            let b = ((r2 - y1 * y1).sqrt() - x1 + (r2 - y2 * y2).sqrt() - x1) * (y2 - y1) / 2.0;
            a + b
        } else {
            let a = circle_segment(r1, (y2 / r1).asin() - (x2 / r1).acos());
            let b = (y2 - (r2 - x2 * x2).sqrt()) * (x2 - (r2 - y2 * y2).sqrt()) / 2.0;
            a - b + c
        }
    }
}

fn circle_segment(radius: f64, theta: f64) -> f64 {
    radius * radius * (theta - theta.sin()) / 2.0
}

google_test_gen!(test, "input.txt", "output.txt");
