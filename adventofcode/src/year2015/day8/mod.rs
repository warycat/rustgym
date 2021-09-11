use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let mut quote = 0;
    let mut escape = 0;
    let mut ascii = 0;
    let mut lines = 0;
    for line in it {
        let mut lit = line.chars();
        while let Some(c) = lit.next() {
            match c {
                '"' => {
                    quote += 1;
                }
                '\\' => {
                    if let Some('x') = lit.next() {
                        ascii += 1;
                        lit.next();
                        lit.next();
                    } else {
                        escape += 1;
                    }
                }
                _ => {}
            }
        }
        lines += 1;
    }
    let res1 = quote + escape + ascii * 3;
    let res2 = lines * 4 + escape * 2 + ascii;
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

adventofcode!(test, "input.txt", "output.txt");
