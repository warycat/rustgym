struct Solution;

impl Solution {
    fn find_kth_positive(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut x = 1;
        let mut i = 0;
        let n = arr.len();
        loop {
            if i < n && x == arr[i] {
                i += 1;
            } else {
                k -= 1;
            }
            if k == 0 {
                break;
            }
            x += 1;
        }
        x
    }
}

#[test]
fn test() {
    let arr = vec![2, 3, 4, 7, 11];
    let k = 5;
    let res = 9;
    assert_eq!(Solution::find_kth_positive(arr, k), res);
    let arr = vec![1, 2, 3, 4];
    let k = 2;
    let res = 6;
    assert_eq!(Solution::find_kth_positive(arr, k), res);
}
