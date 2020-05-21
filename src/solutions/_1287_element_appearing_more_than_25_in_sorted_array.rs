struct Solution;

impl Solution {
    fn find_special_integer(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let t = n / 4;
        for i in 0..n - t {
            if arr[i] == arr[i + t] {
                return arr[i];
            }
        }
        0
    }
}

#[test]
fn test() {
    let arr = vec![1, 2, 2, 6, 6, 6, 6, 7, 10];
    assert_eq!(Solution::find_special_integer(arr), 6);
}
