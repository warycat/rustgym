struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    fn min_refuel_stops(target: i32, start_fuel: i32, mut stations: Vec<Vec<i32>>) -> i32 {
        let mut queue: BinaryHeap<(Reverse<usize>, usize, i32)> = BinaryHeap::new();
        stations.push(vec![target, 0]);
        let n = stations.len();
        let fuel = start_fuel - stations[0][0];
        let mut states: HashMap<(usize, usize), i32> = HashMap::new();
        if fuel >= 0 {
            states.insert((0, 0), fuel);
            queue.push((Reverse(0), 0, fuel));
        }
        while let Some((Reverse(stop), i, fuel)) = queue.pop() {
            if i == n - 1 {
                return stop as i32;
            }
            let dist = stations[i + 1][0] - stations[i][0];
            let take = fuel + stations[i][1] - dist;
            let skip = fuel - dist;
            for &(s, f) in &[(stop + 1, take), (stop, skip)] {
                if f < 0 {
                    continue;
                }
                if let Some(&max_f) = states.get(&(i + 1, s)) {
                    if max_f >= f {
                        continue;
                    }
                }
                *states.entry((i + 1, s)).or_default() = f;
                queue.push((Reverse(s), i + 1, f));
            }
        }
        -1
    }
}

#[test]
fn test() {
    let target = 1;
    let start_fuel = 1;
    let stations = vec_vec_i32![];
    let res = 0;
    assert_eq!(
        Solution::min_refuel_stops(target, start_fuel, stations),
        res
    );
    let target = 100;
    let start_fuel = 1;
    let stations = vec_vec_i32![[10, 100]];
    let res = -1;
    assert_eq!(
        Solution::min_refuel_stops(target, start_fuel, stations),
        res
    );
    let target = 100;
    let start_fuel = 10;
    let stations = vec_vec_i32![[10, 60], [20, 30], [30, 30], [60, 40]];
    let res = 2;
    assert_eq!(
        Solution::min_refuel_stops(target, start_fuel, stations),
        res
    );
}
