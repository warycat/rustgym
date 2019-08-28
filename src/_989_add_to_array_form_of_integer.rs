struct Solution;

impl Solution {
    fn add_to_array_form(mut a: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut i = a.len() - 1;
        while k > 0 {
            let sum = a[i] + k;
            a[i] = sum % 10;
            k = sum / 10;
            if i > 0 {
                i -= 1;
            } else {
                a.insert(0, 0);
            }
        }
        if a.len() > 1 && a[0] == 0 {
            a.remove(0);
        }
        a
    }
}

#[test]
fn test() {
    let a = vec![1, 2, 0, 0];
    let k = 34;
    let res = vec![1, 2, 3, 4];
    assert_eq!(Solution::add_to_array_form(a, k), res);
    let a = vec![2, 7, 4];
    let k = 181;
    let res = vec![4, 5, 5];
    assert_eq!(Solution::add_to_array_form(a, k), res);
    let a = vec![2, 1, 5];
    let k = 806;
    let res = vec![1, 0, 2, 1];
    assert_eq!(Solution::add_to_array_form(a, k), res);
    let a = vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9];
    let k = 1;
    let res = vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(Solution::add_to_array_form(a, k), res);
}
