struct Solution;

impl Solution {
    fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in 1..10 {
            Self::dfs(i, i, &mut res, low, high);
        }
        res.sort_unstable();
        res
    }
    fn dfs(last_digit: i32, cur: i32, all: &mut Vec<i32>, low: i32, high: i32) {
        if cur >= low && cur <= high {
            all.push(cur);
        }
        if cur >= high {
            return;
        }
        if last_digit < 9 {
            Self::dfs(last_digit + 1, cur * 10 + last_digit + 1, all, low, high);
        }
    }
}

#[test]
fn test() {
    let low = 100;
    let high = 300;
    let res = vec![123, 234];
    assert_eq!(Solution::sequential_digits(low, high), res);
    let low = 1000;
    let high = 13000;
    let res = vec![1234, 2345, 3456, 4567, 5678, 6789, 12345];
    assert_eq!(Solution::sequential_digits(low, high), res);
}
