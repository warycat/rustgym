use rustgym_util::*;
use std::collections::HashMap;
use std::fmt::Write;
use std::io::*;

fn solve(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = reader.parse_line();
    let arr: Vec<i32> = reader.parse_vec();
    write!(writer, "{}", sock_merchant(n, arr)).unwrap();
}

fn sock_merchant(n: usize, arr: Vec<i32>) -> usize {
    let mut res = 0;
    let mut count: HashMap<i32, usize> = HashMap::new();
    for i in 0..n {
        *count.entry(arr[i]).or_default() += 1;
    }
    for &v in count.values() {
        res += v / 2;
    }
    res
}

test_gen!(test00, "input00.txt", "output00.txt");
test_gen!(test08, "input08.txt", "output08.txt");
