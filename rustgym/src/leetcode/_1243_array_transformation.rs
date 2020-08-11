struct Solution;

impl Solution {
    fn transform_array(mut array: Vec<i32>) -> Vec<i32> {
        let n = array.len();
        let mut temp = vec![0; n];
        loop {
            if Self::next(&mut array, &mut temp, n) {
                break;
            }
        }
        array
    }
    fn next(arr: &mut Vec<i32>, temp: &mut Vec<i32>, n: usize) -> bool {
        for i in 1..n - 1 {
            temp[i] = arr[i];
            if arr[i] > arr[i - 1] && arr[i] > arr[i + 1] {
                temp[i] -= 1;
            }
            if arr[i] < arr[i - 1] && arr[i] < arr[i + 1] {
                temp[i] += 1;
            }
        }
        let mut res = true;
        for i in 1..n - 1 {
            if arr[i] != temp[i] {
                res = false;
                arr[i] = temp[i];
            }
        }
        res
    }
}

#[test]
fn test() {
    let arr = vec![6, 2, 3, 4];
    let res = vec![6, 3, 3, 4];
    assert_eq!(Solution::transform_array(arr), res);
    let arr = vec![1, 6, 3, 4, 3, 5];
    let res = vec![1, 4, 4, 4, 4, 5];
    assert_eq!(Solution::transform_array(arr), res);
}
