struct Solution;

use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    fn racecar(target: i32) -> i32 {
        let mut queue: VecDeque<(usize, i32, i32)> = VecDeque::new();
        let mut states: HashMap<(i32, i32), usize> = HashMap::new();
        queue.push_back((0, 0, 1));
        states.insert((0, 1), 0);
        while let Some((length, position, speed)) = queue.pop_front() {
            if position == target {
                return length as i32;
            }

            for (p, s) in Self::next(position, speed) {
                if p > target + target / 3 || p < -target / 3 {
                    continue;
                }
                if states.contains_key(&(p, s)) && states[&(p, s)] <= length + 1 {
                    continue;
                }
                *states.entry((p, s)).or_default() = length + 1;
                queue.push_back((length + 1, p, s));
            }
        }
        0
    }

    fn next(position: i32, speed: i32) -> Vec<(i32, i32)> {
        vec![
            (position + speed, speed * 2),
            (position, if speed > 0 { -1 } else { 1 }),
        ]
    }
}

#[test]
fn test() {
    let target = 3;
    let res = 2;
    assert_eq!(Solution::racecar(target), res);
    let target = 6;
    let res = 5;
    assert_eq!(Solution::racecar(target), res);
}
