struct Solution;

impl Solution {
    fn rev_half(s: &mut [char], k: usize) -> &[char] {
        if s.len() <= k {
            s.reverse();
        } else {
            let half = &mut s[0..k];
            half.reverse();
        }
        s
    }
    fn reverse_str(s: String, k: i32) -> String {
        let k: usize = k as usize;
        let mut s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut i = 0;
        while i * 2 * k < n {
            let r = (i + 1) * 2 * k;
            if r < n {
                let ss = &mut s[i * 2 * k..r];
                Self::rev_half(ss, k);
            } else {
                let ss = &mut s[i * 2 * k..n];
                Self::rev_half(ss, k);
            }
            i += 1;
        }
        s.iter().collect()
    }
}

#[test]
fn test() {
    let s = "abcdefg".to_string();
    let k = 2;
    let t = "bacdfeg".to_string();
    assert_eq!(Solution::reverse_str(s, k), t);
}
