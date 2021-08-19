struct Solution;

impl Solution {
    fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let mut lo = 1;
        let mut hi = m * n;
        while lo < hi {
            let mi = lo + (hi - lo) / 2;
            if Self::count(mi, m, n) < k {
                lo = mi + 1;
            } else {
                hi = mi;
            }
        }
        lo
    }

    fn count(x: i32, m: i32, n: i32) -> i32 {
        let mut res = 0;
        for i in 1..=m {
            res += n.min(x / i);
        }
        res
    }
}

#[test]
fn test() {
    let m = 3;
    let n = 3;
    let k = 5;
    let res = 3;
    assert_eq!(Solution::find_kth_number(m, n, k), res);
}
