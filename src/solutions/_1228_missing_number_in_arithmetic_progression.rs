struct Solution;

impl Solution {
    fn missing_number(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let delta = (arr[n - 1] - arr[0]) / n as i32;
        for i in 1..n {
            let v = arr[0] + delta * i as i32;
            if arr[i] != v {
                return v;
            }
        }
        0
    }
}

#[test]
fn test() {
    let arr = vec![5, 7, 11, 13];
    let res = 9;
    assert_eq!(Solution::missing_number(arr), res);
    let arr = vec![15, 13, 12];
    let res = 14;
    assert_eq!(Solution::missing_number(arr), res);
}
