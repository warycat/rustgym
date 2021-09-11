use md5::*;
use rayon::prelude::*;
use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let mut it = reader.lines().map(|l| l.unwrap());
    let line = it.next().unwrap();

    let res1 = search(1000000, &line, "00000");
    let res2 = search(10000000, &line, "000000");
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn search(max: usize, line: &str, prefix: &str) -> usize {
    (0..max)
        .into_par_iter()
        .position_first(|x| {
            let s = format!("{}{}", line, x);
            let digest = format!("{:?}", compute(s));
            digest.starts_with(prefix)
        })
        .unwrap()
}

adventofcode_ignore!(test, "input.txt", "output.txt");
