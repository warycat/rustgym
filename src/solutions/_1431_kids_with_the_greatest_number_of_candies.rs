struct Solution;

impl Solution {
    fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = *candies.iter().max().unwrap();
        candies
            .into_iter()
            .map(|x| x + extra_candies >= max)
            .collect()
    }
}

#[test]
fn test() {
    let candies = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    let res = vec![true, true, true, false, true];
    assert_eq!(Solution::kids_with_candies(candies, extra_candies), res);
    let candies = vec![4, 2, 1, 1, 2];
    let extra_candies = 1;
    let res = vec![true, false, false, false, false];
    assert_eq!(Solution::kids_with_candies(candies, extra_candies), res);
    let candies = vec![12, 1, 12];
    let extra_candies = 10;
    let res = vec![true, false, true];
    assert_eq!(Solution::kids_with_candies(candies, extra_candies), res);
}
