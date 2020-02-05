struct Solution;

impl Solution {
    fn is_prev(prev: &str, next: &str) -> bool {
        let mut i = 0;
        let mut j = 0;
        while i < prev.len() {
            if prev[i..=i] == next[j..=j] {
                i += 1;
                j += 1;
            } else if i == j {
                j += 1;
            } else {
                return false;
            }
        }
        true
    }
    fn longest_str_chain(mut words: Vec<String>) -> i32 {
        let n = words.len();
        let mut v: Vec<i32> = vec![1; n];
        let mut res = 1;
        words.sort_unstable_by_key(|s| s.len());
        for i in 1..n {
            let cur = &words[i];
            let m = cur.len();
            for j in (0..i).rev() {
                let l = words[j].len();
                if l == m {
                    continue;
                } else if l == m - 1 {
                    if Self::is_prev(&words[j], &words[i]) {
                        v[i] = v[j] + 1;
                        res = i32::max(res, v[i]);
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let words: Vec<String> = vec_string!["a", "b", "ba", "bca", "bda", "bdca"];
    assert_eq!(Solution::longest_str_chain(words), 4);
    let words: Vec<String> = vec_string![
        "ksqvsyq",
        "ks",
        "kss",
        "czvh",
        "zczpzvdhx",
        "zczpzvh",
        "zczpzvhx",
        "zcpzvh",
        "zczvh",
        "gr",
        "grukmj",
        "ksqvsq",
        "gruj",
        "kssq",
        "ksqsq",
        "grukkmj",
        "grukj",
        "zczpzfvdhx",
        "gru"
    ];
    assert_eq!(Solution::longest_str_chain(words), 7);
}
