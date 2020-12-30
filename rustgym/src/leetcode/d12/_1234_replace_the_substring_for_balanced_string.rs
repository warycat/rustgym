struct Solution;

impl Solution {
    fn balanced_string(s: String) -> i32 {
        let mut count: Vec<usize> = vec![0; 256];
        let s: Vec<u8> = s.bytes().collect();
        let n = s.len();
        let k = n / 4;
        for i in 0..n {
            count[s[i] as usize] += 1;
        }
        let mut start = 0;
        let mut end = 0;
        let mut res = n;
        while end < n {
            count[s[end] as usize] -= 1;
            end += 1;
            while start < n
                && count[b'Q' as usize] <= k
                && count[b'W' as usize] <= k
                && count[b'E' as usize] <= k
                && count[b'R' as usize] <= k
            {
                res = res.min(end - start);
                count[s[start] as usize] += 1;
                start += 1;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let s = "QWER".to_string();
    let res = 0;
    assert_eq!(Solution::balanced_string(s), res);
    let s = "QQWE".to_string();
    let res = 1;
    assert_eq!(Solution::balanced_string(s), res);
    let s = "QQQW".to_string();
    let res = 2;
    assert_eq!(Solution::balanced_string(s), res);
    let s = "QQQQ".to_string();
    let res = 3;
    assert_eq!(Solution::balanced_string(s), res);
}
