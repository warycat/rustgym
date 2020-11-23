struct Solution;

impl Solution {
    fn find_kth_number(n: i32, k: i32) -> i32 {
        let n = n as i64;
        let mut prefix = 1;
        let mut k = k as i64 - 1;
        while k > 0 {
            let mut first = prefix;
            let mut last = prefix + 1;
            let mut count = 0;
            while first <= n {
                count += last.min(n + 1) - first;
                first *= 10;
                last *= 10;
            }
            if count <= k {
                k -= count;
                prefix += 1;
            } else {
                k -= 1;
                prefix *= 10;
            }
        }
        prefix as i32
    }
}

#[test]
fn test() {
    let n = 13;
    let k = 2;
    let res = 10;
    assert_eq!(Solution::find_kth_number(n, k), res);
    let n = 2;
    let k = 2;
    let res = 2;
    assert_eq!(Solution::find_kth_number(n, k), res);
    let n = 10;
    let k = 3;
    let res = 2;
    assert_eq!(Solution::find_kth_number(n, k), res);
}
