struct Solution;

impl Solution {
    fn max_num_of_substrings(s: String) -> Vec<String> {
        let n = s.len();
        let s: Vec<u8> = s.bytes().collect();
        let mut l = vec![std::usize::MAX; 26];
        let mut r = vec![std::usize::MIN; 26];
        for i in 0..n {
            let j = (s[i] - b'a') as usize;
            l[j] = l[j].min(i);
            r[j] = r[j].max(i);
        }
        let mut res = vec![];
        let mut end = 0;
        for i in 0..n {
            let j = (s[i] - b'a') as usize;
            if i == l[j] {
                if let Some(new_end) = Self::check(i, &l, &r, &s) {
                    if new_end < end {
                        res.pop();
                    }
                    res.push(s[i..new_end].iter().map(|&b| b as char).collect::<String>());
                    end = new_end;
                }
            }
        }
        res
    }

    fn check(start: usize, l: &[usize], r: &[usize], s: &[u8]) -> Option<usize> {
        let mut end = r[(s[start] - b'a') as usize] + 1;
        let mut i = start;
        while i < end {
            if l[(s[i] - b'a') as usize] < start {
                return None;
            } else {
                end = end.max(r[(s[i] - b'a') as usize] + 1);
            }
            i += 1;
        }
        Some(end)
    }
}

#[test]
fn test() {
    let s = "adefaddaccc".to_string();
    let res = vec_string!["e", "f", "ccc"];
    assert_eq!(Solution::max_num_of_substrings(s), res);
}
