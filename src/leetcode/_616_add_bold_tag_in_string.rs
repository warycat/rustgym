struct Solution;

impl Solution {
    fn add_bold_tag(s: String, dict: Vec<String>) -> String {
        let n = s.len();
        let mut tag: Vec<bool> = vec![false; n];
        for p in dict {
            let mut start = 0;
            let m = p.len();
            while let Some(offset) = s[start..].find(&p) {
                for i in 0..m {
                    tag[start + offset + i] = true;
                }
                start += offset + 1;
            }
        }
        let s: Vec<char> = s.chars().collect();
        let mut res = "".to_string();
        let mut in_tag = false;
        for i in 0..n {
            if in_tag != tag[i] {
                res += if in_tag { "</b>" } else { "<b>" };
                in_tag = !in_tag;
            }
            res.push(s[i]);
        }
        if in_tag {
            res += "</b>";
        }
        res
    }
}

#[test]
fn test() {
    let s = "abcxyz123".to_string();
    let dict = vec_string!["abc", "123"];
    let res = "<b>abc</b>xyz<b>123</b>".to_string();
    assert_eq!(Solution::add_bold_tag(s, dict), res);
    let s = "aaabbcc".to_string();
    let dict = vec_string!["aaa", "aab", "bc"];
    let res = "<b>aaabbc</b>c".to_string();
    assert_eq!(Solution::add_bold_tag(s, dict), res);
    let s = "aaabbcc".to_string();
    let dict = vec_string!["a", "b", "c"];
    let res = "<b>aaabbcc</b>".to_string();
    assert_eq!(Solution::add_bold_tag(s, dict), res);
}
