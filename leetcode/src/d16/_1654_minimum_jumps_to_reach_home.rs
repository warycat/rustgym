struct Solution;

use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;

impl Solution {
    fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        let mut visited: HashSet<(i32, bool)> = HashSet::new();
        let mut queue: VecDeque<(i32, bool, i32)> = VecDeque::new();
        let max_pos = forbidden.iter().max().unwrap_or(&0) + a + b + x;
        let forbidden: HashSet<i32> = HashSet::from_iter(forbidden);
        visited.insert((0, false));
        queue.push_back((0, false, 0));
        while let Some((pos, backjump, step)) = queue.pop_front() {
            if pos == x {
                return step;
            }
            let next_pos = pos + a;
            if next_pos <= max_pos
                && !forbidden.contains(&(next_pos))
                && visited.insert((next_pos, false))
            {
                queue.push_back((next_pos, false, step + 1));
            }
            if !backjump {
                let next_pos = pos - b;
                if next_pos >= 0
                    && !forbidden.contains(&(next_pos))
                    && visited.insert((next_pos, true))
                {
                    queue.push_back((next_pos, true, step + 1));
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    let forbidden = vec![14, 4, 18, 1, 15];
    let a = 3;
    let b = 15;
    let x = 9;
    let res = 3;
    assert_eq!(Solution::minimum_jumps(forbidden, a, b, x), res);
    let forbidden = vec![8, 3, 16, 6, 12, 20];
    let a = 15;
    let b = 13;
    let x = 11;
    let res = -1;
    assert_eq!(Solution::minimum_jumps(forbidden, a, b, x), res);
    let forbidden = vec![1, 6, 2, 14, 5, 17, 4];
    let a = 16;
    let b = 9;
    let x = 7;
    let res = 2;
    assert_eq!(Solution::minimum_jumps(forbidden, a, b, x), res);
}
