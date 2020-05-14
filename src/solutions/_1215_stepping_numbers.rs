struct Solution;

impl Solution {
    fn count_stepping_numbers(low: i32, high: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in 0..10 {
            Self::dfs(i, i, &mut res, low as i64, high as i64);
        }
        res.sort_unstable();
        res
    }

    fn dfs(last_digit: i64, cur: i64, all: &mut Vec<i32>, low: i64, high: i64) {
        if cur >= low && cur <= high {
            all.push(cur as i32);
        }
        if cur == 0 || cur > high {
            return;
        }
        if last_digit > 0 {
            Self::dfs(last_digit - 1, cur * 10 + last_digit - 1, all, low, high);
        }
        if last_digit < 9 {
            Self::dfs(last_digit + 1, cur * 10 + last_digit + 1, all, low, high);
        }
    }
}

#[test]
fn test() {
    let low = 0;
    let high = 21;
    let res = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 21];
    assert_eq!(Solution::count_stepping_numbers(low, high), res);
}
