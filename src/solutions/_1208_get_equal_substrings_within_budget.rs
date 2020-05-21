struct Solution;

impl Solution {
    fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let n = s.len();
        let mut cost: Vec<i32> = vec![0; n];
        for i in 0..n {
            cost[i] = (s[i] as i32 - t[i] as i32).abs();
        }
        let mut start = 0;
        let mut end = 0;
        let mut res = 0;
        let mut sum = 0;
        while end < n {
            if sum <= max_cost {
                sum += cost[end];
                end += 1;
            } else {
                sum -= cost[start];
                start += 1;
            }
            if sum <= max_cost {
                res = res.max(end - start);
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let s = "abcd".to_string();
    let t = "cdef".to_string();
    let max_cost = 3;
    let res = 1;
    assert_eq!(Solution::equal_substring(s, t, max_cost), res);
    let s = "abcd".to_string();
    let t = "acde".to_string();
    let max_cost = 0;
    let res = 1;
    assert_eq!(Solution::equal_substring(s, t, max_cost), res);
}
