struct Solution;

impl Solution {
    fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut count: Vec<i32> = vec![1; n];
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                count[i] = count[i].max(count[i - 1] + 1);
            }
        }
        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                count[i] = count[i].max(count[i + 1] + 1);
            }
        }
        count.into_iter().sum()
    }
}

#[test]
fn test() {
    let ratings = vec![1, 0, 2];
    let res = 5;
    assert_eq!(Solution::candy(ratings), res);
    let ratings = vec![1, 2, 2];
    let res = 4;
    assert_eq!(Solution::candy(ratings), res);
    let ratings = vec![1, 3, 2, 2, 1];
    let res = 7;
    assert_eq!(Solution::candy(ratings), res);
}
