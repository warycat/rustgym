struct Solution;

impl Solution {
    fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let n = arr.len();
        let k = k as usize;
        let mut l = 0;
        let mut r = n - k;
        while l < r {
            let m = l + (r - l) / 2;
            if x - arr[m] > arr[m + k] - x {
                l = m + 1;
            } else {
                r = m;
            }
        }
        arr[l..l + k].to_vec()
    }
}

#[test]
fn test() {
    let arr = vec![1, 2, 3, 4, 5];
    let k = 4;
    let x = 3;
    let res = vec![1, 2, 3, 4];
    assert_eq!(Solution::find_closest_elements(arr, k, x), res);
    let arr = vec![1, 2, 3, 4, 5];
    let k = 4;
    let x = -1;
    let res = vec![1, 2, 3, 4];
    assert_eq!(Solution::find_closest_elements(arr, k, x), res);
    let arr = vec![1, 1, 1, 10, 10, 10];
    let k = 1;
    let x = 9;
    let res = vec![10];
    assert_eq!(Solution::find_closest_elements(arr, k, x), res);
}
