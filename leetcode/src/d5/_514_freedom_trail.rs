struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    fn find_rotate_steps(ring: String, key: String) -> i32 {
        let n = ring.len();
        let m = key.len();
        let ring: Vec<char> = ring.chars().collect();
        let key: Vec<char> = key.chars().collect();
        let mut queue: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
        let mut dist = vec![vec![std::i32::MAX; n]; m];
        queue.push((Reverse(0), 0, 0));
        while let Some((Reverse(step), i, size)) = queue.pop() {
            if size == m {
                return step;
            }
            for j in 0..n {
                if ring[j] == key[size] {
                    let d = step + Self::rotate(i as i32, j as i32, n as i32) + 1;
                    if d < dist[size][j] {
                        dist[size][j] = d;
                        queue.push((Reverse(d), j, size + 1));
                    }
                }
            }
        }
        0
    }

    fn rotate(i: i32, j: i32, n: i32) -> i32 {
        let left = i - j;
        let right = j - i;
        let left = if left < 0 { left + n } else { left };
        let right = if right < 0 { right + n } else { right };
        left.min(right)
    }
}

#[test]
fn test() {
    let ring = "godding".to_string();
    let key = "gd".to_string();
    let res = 4;
    assert_eq!(Solution::find_rotate_steps(ring, key), res);
}
