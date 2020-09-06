struct Solution;

impl Solution {
    fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut l = 0;
        while l + 1 < n && arr[l] <= arr[l + 1] {
            l += 1;
        }
        if l == n - 1 {
            return 0;
        }
        let mut r = n - 1;
        while r > 0 && arr[r - 1] <= arr[r] {
            r -= 1;
        }
        let mut res = (n - 1 - l).min(r);
        let mut i = 0;
        let mut j = r;
        while i <= l && j < n {
            if arr[i] <= arr[j] {
                res = res.min(j - i - 1);
                i += 1;
            } else {
                j += 1;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let arr = vec![1, 2, 3, 10, 4, 2, 3, 5];
    let res = 3;
    assert_eq!(Solution::find_length_of_shortest_subarray(arr), res);
    let arr = vec![5, 4, 3, 2, 1];
    let res = 4;
    assert_eq!(Solution::find_length_of_shortest_subarray(arr), res);
    let arr = vec![1, 2, 3];
    let res = 0;
    assert_eq!(Solution::find_length_of_shortest_subarray(arr), res);
    let arr = vec![1];
    let res = 0;
    assert_eq!(Solution::find_length_of_shortest_subarray(arr), res);
}
