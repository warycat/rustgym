use rustgym_util::*;
use std::collections::HashSet;
use std::fmt::Write;
use std::io::*;
use std::iter::FromIterator;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let mut it = reader.lines().map(|l| l.unwrap()).peekable();
    let valid_set: HashSet<String> =
        HashSet::from_iter(vec_string!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);
    let mut res1 = 0;
    let mut res2 = 0;
    while it.peek().is_some() {
        let mut set1: HashSet<String> = HashSet::new();
        let mut set2: HashSet<String> = HashSet::new();
        while let Some(line) = it.next() {
            if line.is_empty() {
                break;
            } else {
                for pair in line.split_whitespace() {
                    let mut pit = pair.split(':');
                    let field = pit.next().unwrap();
                    let value = pit.next().unwrap();
                    set1.insert(field.to_string());
                    if match field {
                        "byr" => byr(value),
                        "iyr" => iyr(value),
                        "eyr" => eyr(value),
                        "hgt" => hgt(value),
                        "hcl" => hcl(value),
                        "ecl" => ecl(value),
                        "pid" => pid(value),
                        _ => false,
                    } {
                        set2.insert(field.to_string());
                    }
                }
            }
        }
        if set1.intersection(&valid_set).count() == 7 {
            res1 += 1;
        }
        if set2.intersection(&valid_set).count() == 7 {
            res2 += 1;
        }
    }
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn byr(value: &str) -> bool {
    if value.len() != 4 {
        return false;
    }
    if let Ok(year) = value.parse::<i32>() {
        (1920..=2002).contains(&year)
    } else {
        false
    }
}

fn iyr(value: &str) -> bool {
    if value.len() != 4 {
        return false;
    }
    if let Ok(year) = value.parse::<i32>() {
        (2010..=2020).contains(&year)
    } else {
        false
    }
}

fn eyr(value: &str) -> bool {
    if value.len() != 4 {
        return false;
    }
    if let Ok(year) = value.parse::<i32>() {
        (2020..=2030).contains(&year)
    } else {
        false
    }
}

fn hgt(value: &str) -> bool {
    let mut num = 0;
    let mut it = value.chars().peekable();
    while let Some('0'..='9') = it.peek() {
        let c = it.next().unwrap();
        num *= 10;
        num += (c as u8 - b'0') as i32;
    }
    let suffix: String = it.collect();
    if suffix == "cm" && 150 <= num && num <= 193 {
        return true;
    }
    if suffix == "in" && 59 <= num && num <= 76 {
        return true;
    }
    false
}

fn hcl(value: &str) -> bool {
    if value.len() != 7 {
        return false;
    }
    let mut it = value.chars();
    let c = it.next().unwrap();
    if c != '#' {
        return false;
    }
    for _ in 0..6 {
        let c = it.next().unwrap();
        if !(('0'..='9').contains(&c) || ('a'..='f').contains(&c)) {
            return false;
        }
    }
    true
}

fn ecl(value: &str) -> bool {
    if value == "amb" {
        return true;
    }
    if value == "blu" {
        return true;
    }
    if value == "brn" {
        return true;
    }
    if value == "gry" {
        return true;
    }
    if value == "grn" {
        return true;
    }
    if value == "hzl" {
        return true;
    }
    if value == "oth" {
        return true;
    }
    false
}

fn pid(value: &str) -> bool {
    if value.len() != 9 {
        return false;
    }
    let mut it = value.chars();
    for _ in 0..9 {
        let c = it.next().unwrap();
        if !('0'..='9').contains(&c) {
            return false;
        }
    }
    true
}

adventofcode!(test, "input.txt", "output.txt");
