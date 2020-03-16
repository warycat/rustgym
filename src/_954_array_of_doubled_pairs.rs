struct Solution;
use std::cmp::Ordering::*;

impl Solution {
    fn can_reorder_doubled(a: Vec<i32>) -> bool {
        let mut nonzero: Vec<Vec<i32>> = vec![vec![]; 2];
        let mut zero: usize = 0;
        for x in a {
            match x.cmp(&0) {
                Equal => {
                    zero += 1;
                }
                Less => {
                    nonzero[0].push(-x);
                }
                Greater => {
                    nonzero[1].push(x);
                }
            }
        }
        if zero % 2 != 0 || nonzero[0].len() % 2 != 0 || nonzero[1].len() % 2 != 0 {
            return false;
        }
        for i in 0..2 {
            nonzero[i].sort_unstable();
            let size = nonzero[i].len();
            let mut fast = 0;
            for slow in 0..size {
                if nonzero[i][slow] == 0 {
                    continue;
                } else {
                    while fast < size && nonzero[i][fast] != 2 * nonzero[i][slow] {
                        fast += 1;
                    }
                    if fast == size {
                        return false;
                    } else {
                        nonzero[i][fast] = 0;
                    }
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    let a = vec![3, 1, 3, 6];
    let res = false;
    assert_eq!(Solution::can_reorder_doubled(a), res);
    let a = vec![2, 1, 2, 6];
    let res = false;
    assert_eq!(Solution::can_reorder_doubled(a), res);
    let a = vec![4, -2, 2, -4];
    let res = true;
    assert_eq!(Solution::can_reorder_doubled(a), res);
    let a = vec![1, 2, 4, 16, 8, 4];
    let res = false;
    assert_eq!(Solution::can_reorder_doubled(a), res);
    let a = vec![1, 2, 1, -8, 8, -4, 4, -4, 2, -2];
    let res = true;
    assert_eq!(Solution::can_reorder_doubled(a), res);
}
