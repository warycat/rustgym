use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let mut res1 = 0;
    let mut res2 = 0;
    for line in it {
        let mut cit = line.chars();
        let mut lo = 0;
        let mut hi = 0;
        while let Some(c) = cit.next() {
            if c == '-' {
                break;
            } else {
                lo *= 10;
                lo += (c as u8 - b'0') as usize;
            }
        }
        while let Some(c) = cit.next() {
            if c == ' ' {
                break;
            } else {
                hi *= 10;
                hi += (c as u8 - b'0') as usize;
            }
        }
        let mut ch = ' ';
        if let Some(c) = cit.next() {
            ch = c;
        }
        cit.next();
        cit.next();
        let mut index = 0;
        let mut count1 = 0;
        let mut count2 = 0;
        while let Some(c) = cit.next() {
            index += 1;
            if c == ch {
                count1 += 1;
                if index == lo || index == hi {
                    count2 += 1;
                }
            }
        }
        if count1 >= lo && count1 <= hi {
            res1 += 1;
        }
        if count2 == 1 {
            res2 += 1;
        }
    }
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

adventofcode!(test, "input.txt", "output.txt");
