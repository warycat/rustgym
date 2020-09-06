struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn num_ways(s: String) -> i32 {
        let n = s.len();
        let s: Vec<i32> = s.chars().map(|c| if c == '1' { 1 } else { 0 }).collect();
        let m: i32 = s.iter().sum();
        if m == 0 {
            return (((n - 1) as i64 * (n - 2) as i64) / 2 % MOD) as i32;
        }
        if m % 3 != 0 {
            return 0;
        }
        let mut indexes = vec![];
        let k = m / 3;
        let mut sum = 0;
        for i in 0..n {
            if s[i] == 1 && sum % k == 0 {
                indexes.push(i);
            }
            sum += s[i];
            if s[i] == 1 && sum % k == 0 {
                indexes.push(i);
            }
        }
        (((indexes[2] - indexes[1]) as i64 * (indexes[4] - indexes[3]) as i64) % MOD) as i32
    }
}

#[test]
fn test() {
    let s = "10101".to_string();
    let res = 4;
    assert_eq!(Solution::num_ways(s), res);
    let s = "1001".to_string();
    let res = 0;
    assert_eq!(Solution::num_ways(s), res);
    let s = "0000".to_string();
    let res = 3;
    assert_eq!(Solution::num_ways(s), res);
    let s = "100100010100110".to_string();
    let res = 12;
    assert_eq!(Solution::num_ways(s), res);
}
