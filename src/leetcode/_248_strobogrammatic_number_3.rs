struct Solution;

impl Solution {
    fn strobogrammatic_in_range(low: String, high: String) -> i32 {
        let mut res = 0;
        let low: Vec<char> = low.chars().collect();
        let high: Vec<char> = high.chars().collect();
        for size in low.len()..=high.len() {
            let mut cur = vec!['0'; size];
            Self::dfs(size, 0, &mut cur, &mut res, &low, &high);
        }
        res
    }

    fn dfs(
        size: usize,
        start: usize,
        cur: &mut Vec<char>,
        all: &mut i32,
        low: &[char],
        high: &[char],
    ) {
        if start >= (1 + size) / 2 {
            if size > 1 && cur[0] == '0' {
                return;
            }
            if size == low.len() && cur.as_slice() < low {
                return;
            }
            if size == high.len() && cur.as_slice() > high {
                return;
            }
            *all += 1;
        } else {
            if start == size - 1 - start {
                for &c in &['0', '1', '8'] {
                    cur[start] = c;
                    Self::dfs(size, start + 1, cur, all, low, high);
                }
            } else {
                for &(a, b) in &[('0', '0'), ('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')] {
                    cur[start] = a;
                    cur[size - 1 - start] = b;
                    Self::dfs(size, start + 1, cur, all, low, high);
                }
            }
        }
    }
}

#[test]
fn test() {
    let low = "50".to_string();
    let high = "100".to_string();
    let res = 3;
    assert_eq!(Solution::strobogrammatic_in_range(low, high), res);
    let low = "0".to_string();
    let high = "0".to_string();
    let res = 1;
    assert_eq!(Solution::strobogrammatic_in_range(low, high), res);
}
