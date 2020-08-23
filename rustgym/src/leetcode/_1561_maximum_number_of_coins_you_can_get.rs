struct Solution;

impl Solution {
    fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();
        let mut res = 0;
        let n = piles.len() / 3;
        for i in 0..n {
            res += piles[i * 2 + n];
        }
        res
    }
}

#[test]
fn test() {
    let piles = vec![2, 4, 1, 2, 7, 8];
    let res = 9;
    assert_eq!(Solution::max_coins(piles), res);
    let piles = vec![2, 4, 5];
    let res = 4;
    assert_eq!(Solution::max_coins(piles), res);
    let piles = vec![9, 8, 7, 6, 5, 1, 2, 3, 4];
    let res = 18;
    assert_eq!(Solution::max_coins(piles), res);
}
