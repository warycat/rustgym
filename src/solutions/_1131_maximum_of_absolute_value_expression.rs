struct Solution;

impl Solution {
    fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let n = arr1.len();
        let max1 = (0..n).map(|i| arr1[i] + arr2[i] + i as i32).max().unwrap();
        let min1 = (0..n).map(|i| arr1[i] + arr2[i] + i as i32).min().unwrap();
        let max2 = (0..n).map(|i| arr1[i] + arr2[i] - i as i32).max().unwrap();
        let min2 = (0..n).map(|i| arr1[i] + arr2[i] - i as i32).min().unwrap();
        let max3 = (0..n).map(|i| arr1[i] - arr2[i] + i as i32).max().unwrap();
        let min3 = (0..n).map(|i| arr1[i] - arr2[i] + i as i32).min().unwrap();
        let max4 = (0..n).map(|i| arr1[i] - arr2[i] - i as i32).max().unwrap();
        let min4 = (0..n).map(|i| arr1[i] - arr2[i] - i as i32).min().unwrap();
        vec![max1 - min1, max2 - min2, max3 - min3, max4 - min4]
            .into_iter()
            .max()
            .unwrap()
    }
}

#[test]
fn test() {
    let arr1 = vec![1, 2, 3, 4];
    let arr2 = vec![-1, 4, 5, 6];
    let res = 13;
    assert_eq!(Solution::max_abs_val_expr(arr1, arr2), res);
    let arr1 = vec![1, -2, -5, 0, 10];
    let arr2 = vec![0, -2, -1, -7, -4];
    let res = 20;
    assert_eq!(Solution::max_abs_val_expr(arr1, arr2), res);
}
