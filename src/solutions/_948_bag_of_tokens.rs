struct Solution;

impl Solution {
    fn bag_of_tokens_score(mut tokens: Vec<i32>, mut p: i32) -> i32 {
        let n = tokens.len();
        if n == 0 {
            return 0;
        }
        tokens.sort_unstable();
        let mut l = 0;
        let mut r = n - 1;
        let mut point = 0;
        let mut res = 0;
        while l <= r {
            if tokens[l] <= p {
                p -= tokens[l];
                point += 1;
                res = res.max(point);
                l += 1;
            } else {
                if point > 0 {
                    p += tokens[r];
                    point -= 1;
                    r -= 1;
                } else {
                    break;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let tokens = vec![100];
    let p = 50;
    let res = 0;
    assert_eq!(Solution::bag_of_tokens_score(tokens, p), res);
    let tokens = vec![100, 200];
    let p = 150;
    let res = 1;
    assert_eq!(Solution::bag_of_tokens_score(tokens, p), res);
    let tokens = vec![100, 200, 300, 400];
    let p = 200;
    let res = 2;
    assert_eq!(Solution::bag_of_tokens_score(tokens, p), res);
}
