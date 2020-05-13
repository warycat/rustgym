struct Solution;

impl Solution {
    fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l = 1;
        let mut r = 1_000_000_000;
        while l < r {
            let m = (l + r) / 2;
            let mut sum = 0;
            for p in &piles {
                sum += if p % m == 0 { p / m } else { p / m + 1 };
            }
            if sum > h {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }
}

#[test]
fn test() {
    let piles = vec![3, 6, 7, 11];
    let h = 8;
    let res = 4;
    assert_eq!(Solution::min_eating_speed(piles, h), res);
}
