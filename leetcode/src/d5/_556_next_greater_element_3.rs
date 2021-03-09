struct Solution;

impl Solution {
    fn next_greater_element(n: i32) -> i32 {
        let mut s: Vec<char> = format!("{}", n).chars().collect();
        let n = s.len();
        let mut l = n;
        for i in (0..n - 1).rev() {
            if s[i] < s[i + 1] {
                l = i;
                break;
            }
        }
        if l == n {
            return -1;
        }
        let mut max = s[l + 1];
        let mut r = l + 1;
        for i in l + 2..n {
            if s[i] > s[l] && s[i] < max {
                max = s[i];
                r = i;
            }
        }
        s.swap(l, r);
        s[l + 1..].sort_unstable();
        s.iter().collect::<String>().parse::<i32>().unwrap_or(-1)
    }
}

#[test]
fn test() {
    let n = 12;
    let res = 21;
    assert_eq!(Solution::next_greater_element(n), res);
    let n = 21;
    let res = -1;
    assert_eq!(Solution::next_greater_element(n), res);
    let n = 1_999_999_999;
    let res = -1;
    assert_eq!(Solution::next_greater_element(n), res);
}
