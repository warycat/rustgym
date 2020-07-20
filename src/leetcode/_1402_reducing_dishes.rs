struct Solution;

impl Solution {
    fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort_unstable();
        let n = satisfaction.len();
        let mut total = 0;
        let mut res = 0;
        for i in (0..n).rev() {
            if satisfaction[i] + total < 0 {
                break;
            } else {
                total += satisfaction[i];
                res += total;
            }
        }
        res
    }
}

#[test]
fn test() {
    let satisfaction = vec![-1, -8, 0, 5, -9];
    let res = 14;
    assert_eq!(Solution::max_satisfaction(satisfaction), res);
    let satisfaction = vec![4, 3, 2];
    let res = 20;
    assert_eq!(Solution::max_satisfaction(satisfaction), res);
    let satisfaction = vec![-1, -4, -5];
    let res = 0;
    assert_eq!(Solution::max_satisfaction(satisfaction), res);
    let satisfaction = vec![-2, 5, -1, 0, 3, -3];
    let res = 35;
    assert_eq!(Solution::max_satisfaction(satisfaction), res);
}
