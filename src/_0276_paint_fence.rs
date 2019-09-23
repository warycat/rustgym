struct Solution;

impl Solution {
    fn num_ways(n: i32, k: i32) -> i32 {
        if n == 0 || k == 0 {
            return 0;
        }
        if n == 1 {
            return k;
        }
        if k == 1 {
            return 1;
        }
        let mut same = k;
        let mut diff = k * (k - 1);
        for _ in 2..n {
            let t_same = same;
            let t_diff = diff;
            same = t_diff;
            diff = (t_same + t_diff) * (k - 1);
        }
        same + diff
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_ways(3, 2), 6);
}
