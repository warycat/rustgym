use rustgym_util::*;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io::*;

fn solve(case_no: usize, reader: &mut impl BufRead, writer: &mut impl Write) {
    let args: Vec<usize> = reader.parse_vec();
    let r = args[0];
    let c = args[1];
    let mut queue: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();
    let a: Vec<Vec<i64>> = reader.parse_mat(r);
    for i in 0..r {
        for j in 0..c {
            queue.push((a[i][j], i, j));
        }
    }
    let mut b: Vec<Vec<i64>> = a.clone();
    let mut res = 0;
    while let Some((h, i, j)) = queue.pop() {
        res += h - a[i][j];
        if i > 0 && h - 1 > b[i - 1][j] {
            b[i - 1][j] = h - 1;
            queue.push((h - 1, i - 1, j));
        }
        if j > 0 && h - 1 > b[i][j - 1] {
            b[i][j - 1] = h - 1;
            queue.push((h - 1, i, j - 1));
        }
        if i + 1 < r && h - 1 > b[i + 1][j] {
            b[i + 1][j] = h - 1;
            queue.push((h - 1, i + 1, j));
        }
        if j + 1 < c && h - 1 > b[i][j + 1] {
            b[i][j + 1] = h - 1;
            queue.push((h - 1, i, j + 1));
        }
    }
    writeln!(writer, "Case #{}: {}", case_no, res).unwrap();
}

google_test_gen!(test, "input.txt", "output.txt");
