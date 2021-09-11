use rustgym_util::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let mut it = reader.lines().map(|l| l.unwrap());
    let mut valid_ranges: Vec<[i32; 4]> = vec![];
    let mut departure_fields: Vec<usize> = vec![];
    let mut fid = 0;
    while let Some(line) = it.next() {
        if line.is_empty() {
            break;
        } else {
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts[0] == "departure" {
                departure_fields.push(fid);
            }
            let n = parts.len();
            let r1: Vec<i32> = parts[n - 3]
                .split('-')
                .map(|w| w.parse::<i32>().unwrap())
                .collect();
            let r2: Vec<i32> = parts[n - 1]
                .split('-')
                .map(|w| w.parse::<i32>().unwrap())
                .collect();
            valid_ranges.push([r1[0], r1[1], r2[0], r2[1]]);
        }
        fid += 1;
    }
    it.next().unwrap();
    let your_ticket: Vec<i32> = it
        .next()
        .unwrap()
        .split(',')
        .map(|w| w.parse::<i32>().unwrap())
        .collect();
    it.next().unwrap();
    it.next().unwrap();
    let mut nearby_tickets: Vec<Vec<i32>> = vec![];
    while let Some(line) = it.next() {
        let mut ticket = vec![];
        for t in line.split(',').map(|w| w.parse::<i32>().unwrap()) {
            ticket.push(t);
        }
        nearby_tickets.push(ticket);
    }
    let res1 = error_rate(&valid_ranges, &nearby_tickets);
    let valid_nearby_tickets = valid_tickets(&valid_ranges, &nearby_tickets);
    let res2 = departure_product(
        &valid_ranges,
        &valid_nearby_tickets,
        your_ticket,
        departure_fields,
    );
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn error_rate(valid_ranges: &[[i32; 4]], nearby_tickets: &[Vec<i32>]) -> i32 {
    let mut res = 0;
    for ticket in nearby_tickets {
        for &t in ticket {
            if !valid_ranges
                .iter()
                .any(|r| r[0] <= t && t <= r[1] || r[2] <= t && t <= r[3])
            {
                res += t;
            }
        }
    }
    res
}

fn valid_tickets(valid_ranges: &[[i32; 4]], nearby_tickets: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut res = vec![];
    'outer: for ticket in nearby_tickets {
        for &t in ticket {
            if !valid_ranges
                .iter()
                .any(|r| r[0] <= t && t <= r[1] || r[2] <= t && t <= r[3])
            {
                continue 'outer;
            }
        }
        res.push(ticket.to_vec());
    }
    res
}

fn departure_product(
    valid_ranges: &[[i32; 4]],
    valid_nearby_tickets: &[Vec<i32>],
    your_ticket: Vec<i32>,
    departure_fields: Vec<usize>,
) -> i64 {
    let n = your_ticket.len();
    let mut possible_set: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for i in 0..n {
        let range = &valid_ranges[i];
        for j in 0..n {
            if valid_nearby_tickets
                .iter()
                .all(|ticket| in_range(ticket[j], range))
            {
                possible_set[i].insert(j);
            }
        }
    }
    let mut unknown: HashSet<usize> = (0..n).collect();
    let mut field_map: HashMap<usize, usize> = HashMap::new();
    while !unknown.is_empty() {
        let unknown_copy: Vec<usize> = unknown.iter().copied().collect();
        for i in unknown_copy {
            if possible_set[i].len() == 1 {
                let id: usize = *possible_set[i].iter().next().unwrap();
                field_map.insert(i, id);
                for set in possible_set.iter_mut() {
                    set.remove(&id);
                }
                unknown.remove(&i);
            }
        }
    }
    departure_fields
        .into_iter()
        .map(|id| your_ticket[field_map[&id]] as i64)
        .product()
}

fn in_range(t: i32, range: &[i32; 4]) -> bool {
    range[0] <= t && t <= range[1] || range[2] <= t && t <= range[3]
}

adventofcode!(test, "input.txt", "output.txt");
