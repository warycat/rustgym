struct Solution;

impl Solution {
    fn min_increment_for_unique(mut a: Vec<i32>) -> i32 {
        let mut res = 0;
        a.sort_unstable();
        let mut prev: Option<i32> = None;
        for x in a {
            if let Some(y) = prev {
                if x <= y {
                    res += y + 1 - x;
                    prev = Some(y + 1);
                } else {
                    prev = Some(x);
                }
            } else {
                prev = Some(x);
            }
        }
        res
    }
}

#[test]
fn test() {
    let a = vec![1, 2, 2];
    let res = 1;
    assert_eq!(Solution::min_increment_for_unique(a), res);
    let a = vec![3, 2, 1, 2, 1, 7];
    let res = 6;
    assert_eq!(Solution::min_increment_for_unique(a), res);
}
