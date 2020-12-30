struct Solution;

impl Solution {
    fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = card_points.len();
        let mut sum: i32 = card_points.iter().take(k).sum();
        let mut max = sum;
        for i in 0..k {
            sum -= card_points[k - 1 - i];
            sum += card_points[n - 1 - i];
            max = max.max(sum);
        }
        max
    }
}

#[test]
fn test() {
    let card_points = vec![1, 2, 3, 4, 5, 6, 1];
    let k = 3;
    let res = 12;
    assert_eq!(Solution::max_score(card_points, k), res);
    let card_points = vec![2, 2, 2];
    let k = 2;
    let res = 4;
    assert_eq!(Solution::max_score(card_points, k), res);
    let card_points = vec![9, 7, 7, 9, 7, 7, 9];
    let k = 7;
    let res = 55;
    assert_eq!(Solution::max_score(card_points, k), res);
    let card_points = vec![1, 1000, 1];
    let k = 1;
    let res = 1;
    assert_eq!(Solution::max_score(card_points, k), res);
    let card_points = vec![1, 79, 80, 1, 1, 1, 200, 1];
    let k = 3;
    let res = 202;
    assert_eq!(Solution::max_score(card_points, k), res);
}
