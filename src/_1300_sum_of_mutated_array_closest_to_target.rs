struct Solution;

impl Solution {
    fn find_best_value(mut arr: Vec<i32>, mut target: i32) -> i32 {
        arr.sort_unstable();
        let n = arr.len();
        let mut i = 0;
        while i < n && target > arr[i] * (n - i) as i32 {
            target -= arr[i];
            i += 1;
        }
        if i == n {
            return arr[n - 1];
        }
        let res = target / (n - i) as i32;
        if (res + 1) * (n - i) as i32 - target < target - res * (n - i) as i32 {
            res + 1
        } else {
            res
        }
    }
}

#[test]
fn test() {
    let arr = vec![4, 9, 3];
    let target = 10;
    let res = 3;
    assert_eq!(Solution::find_best_value(arr, target), res);
    let arr = vec![2, 3, 5];
    let target = 10;
    let res = 5;
    assert_eq!(Solution::find_best_value(arr, target), res);
    let arr = vec![60864, 25176, 27249, 21296, 20204];
    let target = 56803;
    let res = 11361;
    assert_eq!(Solution::find_best_value(arr, target), res);
}
