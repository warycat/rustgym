struct Solution;

impl Solution {
    fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        arr.windows(3)
            .any(|w| w[0] % 2 == 1 && w[1] % 2 == 1 && w[2] % 2 == 1)
    }
}

#[test]
fn test() {
    let arr = vec![2, 6, 4, 1];
    let res = false;
    assert_eq!(Solution::three_consecutive_odds(arr), res);
    let arr = vec![1, 2, 34, 3, 4, 5, 7, 23, 12];
    let res = true;
    assert_eq!(Solution::three_consecutive_odds(arr), res);
}
