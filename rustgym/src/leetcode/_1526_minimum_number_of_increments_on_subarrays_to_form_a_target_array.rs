struct Solution;

impl Solution {
    fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut prev = 0;
        for x in target {
            if x > prev {
                res += x - prev;
            }
            prev = x;
        }
        res
    }
}

#[test]
fn test() {
    let target = vec![1, 2, 3, 2, 1];
    let res = 3;
    assert_eq!(Solution::min_number_operations(target), res);
    let target = vec![3, 1, 1, 2];
    let res = 4;
    assert_eq!(Solution::min_number_operations(target), res);
    let target = vec![3, 1, 5, 4, 2];
    let res = 7;
    assert_eq!(Solution::min_number_operations(target), res);
    let target = vec![1, 1, 1, 1];
    let res = 1;
    assert_eq!(Solution::min_number_operations(target), res);
}
