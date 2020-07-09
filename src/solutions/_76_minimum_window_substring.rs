struct Solution;

impl Solution {
    fn min_window(s: String, t: String) -> String {
        let mut freq: Vec<usize> = vec![0; 256];
        let mut limit: Vec<usize> = vec![0; 256];
        let mut k = 0;
        let n = s.len();
        for b in t.bytes() {
            if limit[b as usize] == 0 {
                k += 1;
            }
            limit[b as usize] += 1;
        }
        let v: Vec<u8> = s.bytes().collect();
        let mut start = 0;
        let mut end = 0;
        let mut res = (std::usize::MAX, "");
        loop {
            if k == 0 {
                if end - start < res.0 {
                    res = (end - start, &s[start..end]);
                }
                if limit[v[start] as usize] != 0 {
                    if freq[v[start] as usize] == limit[v[start] as usize] {
                        k += 1;
                    }
                    freq[v[start] as usize] -= 1;
                }
                start += 1;
            } else {
                if end == n {
                    break;
                } else {
                    if limit[v[end] as usize] != 0 {
                        freq[v[end] as usize] += 1;
                        if freq[v[end] as usize] == limit[v[end] as usize] {
                            k -= 1;
                        }
                    }
                    end += 1;
                }
            }
        }
        res.1.to_string()
    }
}

#[test]
fn test() {
    let s = "ADOBECODEBANC".to_string();
    let t = "ABC".to_string();
    let res = "BANC".to_string();
    assert_eq!(Solution::min_window(s, t), res);
}
