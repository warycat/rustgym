struct Solution;

impl Solution {
    fn num_decodings(s: String) -> i32 {
        let s: Vec<u8> = s.bytes().map(|b| b - b'0').collect();
        let n = s.len();
        let mut a: Vec<i32> = vec![0; n + 1];
        if n == 0 {
            return 0;
        }
        a[0] = 1;
        a[1] = if s[0] > 0 { 1 } else { 0 };
        for i in 1..n {
            let first = s[i];
            let second = s[i - 1] * 10 + s[i];
            if (1..=9).contains(&first) {
                a[i + 1] += a[i];
            }
            if (10..=26).contains(&second) {
                a[i + 1] += a[i - 1];
            }
        }
        a[n]
    }
}

#[test]
fn test() {
    let s = "12".to_string();
    assert_eq!(Solution::num_decodings(s), 2);
    let s = "226".to_string();
    assert_eq!(Solution::num_decodings(s), 3);
}
