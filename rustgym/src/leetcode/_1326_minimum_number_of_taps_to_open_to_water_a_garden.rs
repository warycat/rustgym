struct Solution;

impl Solution {
    fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut jumps = vec![0; n + 1];
        for i in 0..=n {
            let d = ranges[i];
            let l = 0.max(i as i32 - d) as usize;
            let r = n.min(i + d as usize);
            jumps[l] = jumps[l].max(r - l);
        }
        let mut end = 0;
        let mut reach = 0;
        let mut res = 0;
        for i in 0..n {
            if i > reach {
                return -1;
            }
            reach = reach.max(i + jumps[i]);
            if i == end {
                res += 1;
                end = reach;
            }
        }
        if reach >= n as usize {
            res
        } else {
            -1
        }
    }
}

#[test]
fn test() {
    let n = 5;
    let ranges = vec![3, 4, 1, 1, 0, 0];
    let res = 1;
    assert_eq!(Solution::min_taps(n, ranges), res);
    let n = 3;
    let ranges = vec![0, 0, 0, 0];
    let res = -1;
    assert_eq!(Solution::min_taps(n, ranges), res);
    let n = 7;
    let ranges = vec![1, 2, 1, 0, 2, 1, 0, 1];
    let res = 3;
    assert_eq!(Solution::min_taps(n, ranges), res);
    let n = 8;
    let ranges = vec![4, 0, 0, 0, 0, 0, 0, 0, 4];
    let res = 2;
    assert_eq!(Solution::min_taps(n, ranges), res);
}
