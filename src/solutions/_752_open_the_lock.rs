struct Solution;
use std::collections::HashSet;

impl Solution {
    fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut visited: HashSet<u32> = deadends.into_iter().map(Self::s2x).collect();
        let target = Self::s2x(target);
        let start = 0;
        if visited.contains(&start) {
            return -1;
        }
        let mut begin: HashSet<u32> = HashSet::new();
        let mut end: HashSet<u32> = HashSet::new();
        begin.insert(start);
        end.insert(target);
        let mut level = 0;
        while !begin.is_empty() && !end.is_empty() {
            let mut temp: HashSet<u32> = HashSet::new();
            for x in begin {
                if end.contains(&x) {
                    return level;
                } else {
                    visited.insert(x);
                    for y in Self::adj(x) {
                        if !visited.contains(&y) {
                            temp.insert(y);
                        }
                    }
                }
            }
            level += 1;
            begin = end;
            end = temp
        }
        -1
    }

    fn s2x(s: String) -> u32 {
        let mut res = 0;
        for (i, b) in s.bytes().map(|b| (b - b'0') as u32).enumerate() {
            res |= b << (i * 8);
        }
        res
    }

    fn x2s(x: u32) -> String {
        let mut v: Vec<u8> = vec![0; 4];
        for i in 0..4 {
            v[i] = ((x >> (i * 8)) & 0xff) as u8;
        }
        v.into_iter().map(|b| (b + b'0') as char).collect()
    }

    fn adj(x: u32) -> Vec<u32> {
        let mut res = vec![];
        for i in 0..4 {
            let b1 = (((x >> (i * 8) & 0xff) + 1) % 10) << (i * 8);
            let b2 = (((x >> (i * 8) & 0xff) + 9) % 10) << (i * 8);
            let mask = !(0xff << (i * 8));
            res.push((x & mask) | b1);
            res.push((x & mask) | b2);
        }
        res
    }
}

#[test]
fn test() {
    let deadends = vec_string!["0201", "0101", "0102", "1212", "2002"];
    let target = "0202".to_string();
    let res = 6;
    assert_eq!(Solution::open_lock(deadends, target), res);
}
