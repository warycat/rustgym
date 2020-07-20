struct Solution;

impl Solution {
    fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
        let mut max_count = [0; 26];
        for s in b {
            let mut count = [0; 26];
            for b in s.bytes() {
                count[(b - b'a') as usize] += 1;
            }
            for i in 0..26 {
                max_count[i] = max_count[i].max(count[i]);
            }
        }

        let mut res = vec![];
        'a: for s in a {
            let mut count = [0; 26];
            for b in s.bytes() {
                count[(b - b'a') as usize] += 1;
            }
            for i in 0..26 {
                if count[i] < max_count[i] {
                    continue 'a;
                };
            }
            res.push(s)
        }

        res
    }
}

#[test]
fn test() {
    let a = vec_string!["amazon", "apple", "facebook", "google", "leetcode"];
    let b = vec_string!["e", "o"];
    let res = vec_string!["facebook", "google", "leetcode"];
    assert_eq!(Solution::word_subsets(a, b), res);
    let a = vec_string!["amazon", "apple", "facebook", "google", "leetcode"];
    let b = vec_string!["l", "e"];
    let res = vec_string!["apple", "google", "leetcode"];
    assert_eq!(Solution::word_subsets(a, b), res);
    let a = vec_string!["amazon", "apple", "facebook", "google", "leetcode"];
    let b = vec_string!["e", "oo"];
    let res = vec_string!["facebook", "google"];
    assert_eq!(Solution::word_subsets(a, b), res);
    let a = vec_string!["amazon", "apple", "facebook", "google", "leetcode"];
    let b = vec_string!["lo", "eo"];
    let res = vec_string!["google", "leetcode"];
    assert_eq!(Solution::word_subsets(a, b), res);
    let a = vec_string!["amazon", "apple", "facebook", "google", "leetcode"];
    let b = vec_string!["ec", "oc", "ceo"];
    let res = vec_string!["facebook", "leetcode"];
    assert_eq!(Solution::word_subsets(a, b), res);
}
