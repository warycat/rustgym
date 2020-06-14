struct Solution;

impl Solution {
    fn smallest_range_ii(mut a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        a.sort_unstable();
        let mut max = a[n - 1];
        let mut min = a[0];
        let mut res = max - min;
        for i in 0..n - 1 {
            max = max.max(a[i] + 2 * k);
            min = a[i + 1].min(a[0] + 2 * k);
            res = res.min(max - min);
        }
        res
    }
}

#[test]
fn test() {
    let a = vec![1];
    let k = 0;
    let res = 0;
    assert_eq!(Solution::smallest_range_ii(a, k), res);
    let a = vec![0, 10];
    let k = 2;
    let res = 6;
    assert_eq!(Solution::smallest_range_ii(a, k), res);
    let a = vec![1, 3, 6];
    let k = 3;
    let res = 3;
    assert_eq!(Solution::smallest_range_ii(a, k), res);
    let a = vec![2, 7, 2];
    let k = 1;
    let res = 3;
    assert_eq!(Solution::smallest_range_ii(a, k), res);
}
