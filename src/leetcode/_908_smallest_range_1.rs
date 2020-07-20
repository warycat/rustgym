struct Solution;

impl Solution {
    fn smallest_range_i(a: Vec<i32>, k: i32) -> i32 {
        if a.is_empty() {
            return 0;
        }
        let max = a.iter().max().unwrap();
        let min = a.iter().min().unwrap();
        let diff = max - min - 2 * k;
        if diff > 0 {
            diff
        } else {
            0
        }
    }
}

#[test]
fn test() {
    let a = vec![1];
    let k = 0;
    let res = 0;
    assert_eq!(Solution::smallest_range_i(a, k), res);
    let a = vec![0, 10];
    let k = 2;
    let res = 6;
    assert_eq!(Solution::smallest_range_i(a, k), res);
    let a = vec![1, 3, 6];
    let k = 3;
    let res = 0;
    assert_eq!(Solution::smallest_range_i(a, k), res);
}
