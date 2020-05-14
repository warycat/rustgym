struct Solution;

impl Solution {
    fn num_teams(rating: Vec<i32>) -> i32 {
        let n = rating.len();
        let mut res = 0;
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    if rating[i] < rating[j] && rating[j] < rating[k] {
                        res += 1;
                    }
                    if rating[i] > rating[j] && rating[j] > rating[k] {
                        res += 1;
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let rating = vec![2, 5, 3, 4, 1];
    let res = 3;
    assert_eq!(Solution::num_teams(rating), res);
    let rating = vec![2, 1, 3];
    let res = 0;
    assert_eq!(Solution::num_teams(rating), res);
    let rating = vec![1, 2, 3, 4];
    let res = 4;
    assert_eq!(Solution::num_teams(rating), res);
}
