struct Solution;

impl Solution {
    fn duplicate_zeros(arr: &mut Vec<i32>) {
        let n = arr.len();
        let m = arr.iter().filter(|&x| *x == 0).count();
        let mut i = n - 1;
        let mut j = m + n - 1;
        while i > 0 {
            if arr[i] != 0 {
                if j < n {
                    arr[j] = arr[i];
                }
                i -= 1;
            } else {
                if j < n {
                    arr[j] = 0;
                }
                i -= 1;
                j -= 1;
                if j < n {
                    arr[j] = 0;
                }
            }
            j -= 1;
        }
        if arr[i] != 0 {
            arr[j] = arr[i];
        } else {
            arr[j] = 0;
            j -= 1;
            arr[j] = 0;
        }
    }
}

#[test]
fn test() {
    let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
    let res = vec![1, 0, 0, 2, 3, 0, 0, 4];
    Solution::duplicate_zeros(&mut arr);
    assert_eq!(arr, res);
}
