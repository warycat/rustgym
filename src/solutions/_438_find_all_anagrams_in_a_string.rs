struct Solution;

impl Solution {
    fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut ss: Vec<usize> = vec![0; 26];
        let mut sp: Vec<usize> = vec![0; 26];
        let s: Vec<u8> = s.bytes().collect();
        let p: Vec<u8> = p.bytes().collect();
        if s.len() < p.len() {
            return res;
        }
        for i in 0..p.len() {
            let c = s[i] as usize - 'a' as usize;
            ss[c] += 1;
        }
        for i in 0..p.len() {
            let c = p[i] as usize - 'a' as usize;
            sp[c] += 1;
        }
        if ss == sp {
            res.push(0);
        }
        for i in 1..=(s.len() - p.len()) {
            let c = s[i - 1] as usize - 'a' as usize;
            let d = s[..][i + p.len() - 1] as usize - 'a' as usize;
            ss[c] -= 1;
            ss[d] += 1;
            if ss == sp {
                res.push(i as i32);
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "cbaebabacd".to_string();
    let p = "abc".to_string();
    assert_eq!(Solution::find_anagrams(s, p), vec![0, 6]);
    let s = "abab".to_string();
    let p = "ab".to_string();
    assert_eq!(Solution::find_anagrams(s, p), vec![0, 1, 2]);
}
