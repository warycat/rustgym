struct Solution;

impl Solution {
    fn add_negabinary(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> Vec<i32> {
        let n = arr1.len();
        let m = arr2.len();
        arr1.reverse();
        arr2.reverse();
        let mut carry = 0;
        let mut res = vec![];
        let mut i = 0;
        while i < n.max(m) || carry != 0 {
            if i < n {
                carry += arr1[i];
            }
            if i < m {
                carry += arr2[i];
            }
            res.push(carry & 1);
            carry = -(carry >> 1);
            i += 1;
        }
        while let Some(&0) = res.last() {
            res.pop();
        }
        res.reverse();
        if res.is_empty() {
            vec![0]
        } else {
            res
        }
    }
}

#[test]
fn test() {
    let arr1 = vec![1, 1, 1, 1, 1];
    let arr2 = vec![1, 0, 1];
    let res = vec![1, 0, 0, 0, 0];
    assert_eq!(Solution::add_negabinary(arr1, arr2), res);
    let arr1 = vec![1];
    let arr2 = vec![1];
    let res = vec![1, 1, 0];
    assert_eq!(Solution::add_negabinary(arr1, arr2), res);
    let arr1 = vec![1];
    let arr2 = vec![1, 1];
    let res = vec![0];
    assert_eq!(Solution::add_negabinary(arr1, arr2), res);
}
