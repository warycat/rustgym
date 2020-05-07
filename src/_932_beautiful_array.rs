struct Solution;

impl Solution {
    fn beautiful_array(n: i32) -> Vec<i32> {
        if n == 1 {
            vec![1]
        } else {
            let left = Self::beautiful_array(n / 2);
            let right = Self::beautiful_array((n + 1) / 2);
            left.into_iter()
                .map(|x| x * 2)
                .chain(right.into_iter().map(|x| x * 2 - 1))
                .collect()
        }
    }
}

#[test]
fn test() {
    let n = 4;
    let res = vec![4, 2, 3, 1];
    assert_eq!(Solution::beautiful_array(n), res);
}
