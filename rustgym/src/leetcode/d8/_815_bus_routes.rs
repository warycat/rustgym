struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    fn num_buses_to_destination(routes: Vec<Vec<i32>>, s: i32, t: i32) -> i32 {
        if s == t {
            return 0;
        }
        let mut buses: HashMap<i32, HashSet<usize>> = HashMap::new();
        for i in 0..routes.len() {
            for &j in &routes[i] {
                buses.entry(j).or_default().insert(i);
            }
        }
        let mut queue: VecDeque<i32> = VecDeque::new();
        let mut visited: HashSet<usize> = HashSet::new();
        queue.push_back(s);
        let mut res = 0;
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let u = queue.pop_front().unwrap();
                if u == t {
                    return res;
                }
                let mut stops = HashSet::new();
                for &bus in &buses[&u] {
                    if visited.insert(bus) {
                        for &i in &routes[bus] {
                            stops.insert(i);
                        }
                    }
                }
                for stop in stops {
                    queue.push_back(stop);
                }
            }
            res += 1;
        }
        -1
    }
}

#[test]
fn test() {
    let routes = vec_vec_i32![[1, 2, 7], [3, 6, 7]];
    let s = 1;
    let t = 6;
    let res = 2;
    assert_eq!(Solution::num_buses_to_destination(routes, s, t), res);
}
