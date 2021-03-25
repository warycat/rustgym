use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

fn solve(case_no: usize, reader: &mut impl BufRead, writer: &mut impl Write) {
    let args = reader.parse_vec();
    let r = args[0];
    let c = args[1];
    let a: Vec<Vec<i32>> = reader.parse_mat(r);
    let mut up: Vec<Vec<usize>> = vec![vec![0; c]; r];
    let mut left: Vec<Vec<usize>> = vec![vec![0; c]; r];
    let mut down: Vec<Vec<usize>> = vec![vec![0; c]; r];
    let mut right: Vec<Vec<usize>> = vec![vec![0; c]; r];
    for i in 0..r {
        for j in 0..c {
            if a[i][j] == 1 {
                up[i][j] = 1;
                left[i][j] = 1;
                if i > 0 {
                    up[i][j] += up[i - 1][j];
                }
                if j > 0 {
                    left[i][j] += left[i][j - 1];
                }
            }
        }
    }
    for i in (0..r).rev() {
        for j in (0..c).rev() {
            if a[i][j] == 1 {
                down[i][j] = 1;
                right[i][j] = 1;
                if i + 1 < r {
                    down[i][j] += down[i + 1][j];
                }
                if j + 1 < c {
                    right[i][j] += right[i][j + 1];
                }
            }
        }
    }
    let mut res = 0;
    for i in 0..r {
        for j in 0..c {
            res += count(up[i][j], left[i][j]);
            res += count(up[i][j], right[i][j]);
            res += count(down[i][j], left[i][j]);
            res += count(down[i][j], right[i][j]);
        }
    }
    writeln!(writer, "Case #{}: {}", case_no, res).unwrap();
}

fn count(a: usize, b: usize) -> usize {
    let x = (a / 2).min(b);
    let y = (b / 2).min(a);
    let mut res = 0;
    if x > 1 {
        res += x - 1;
    }
    if y > 1 {
        res += y - 1;
    }
    res
}

google_test_gen!(test, "input.txt", "output.txt");
