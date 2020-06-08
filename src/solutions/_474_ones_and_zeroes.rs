struct Solution;

use std::collections::HashMap;

impl Solution {
    fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut count: Vec<(i32, i32)> = strs
            .iter()
            .map(|s| {
                (
                    s.chars().filter(|&c| c == '0').count() as i32,
                    s.chars().filter(|&c| c == '1').count() as i32,
                )
            })
            .collect();
        count.sort_unstable();
        let mut memo: HashMap<(usize, i32, i32), usize> = HashMap::new();
        Self::dp(0, m, n, &mut memo, &count, strs.len()) as i32
    }

    fn dp(
        start: usize,
        zero: i32,
        one: i32,
        memo: &mut HashMap<(usize, i32, i32), usize>,
        count: &[(i32, i32)],
        n: usize,
    ) -> usize {
        if start == n {
            return 0;
        }
        if let Some(&res) = memo.get(&(start, zero, one)) {
            return res;
        }
        let skip = Self::dp(start + 1, zero, one, memo, count, n);
        let res = if zero >= count[start].0 && one >= count[start].1 {
            let keep = 1 + Self::dp(
                start + 1,
                zero - count[start].0,
                one - count[start].1,
                memo,
                count,
                n,
            );
            skip.max(keep)
        } else {
            skip
        };
        memo.insert((start, zero, one), res);
        res
    }
}

#[test]
fn test() {
    let strs = vec_string!["10", "0001", "111001", "1", "0"];
    let m = 5;
    let n = 3;
    let res = 4;
    assert_eq!(Solution::find_max_form(strs, m, n), res);
    let strs = vec_string!["10", "0", "1"];
    let m = 1;
    let n = 1;
    let res = 2;
    assert_eq!(Solution::find_max_form(strs, m, n), res);
}
