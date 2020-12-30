struct Solution;

impl Solution {
    fn orderly_queue(s: String, k: i32) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let n = s.len();
        if k > 1 {
            s.sort_unstable();
            s.into_iter().collect()
        } else {
            let mut res = s.to_vec();
            for i in 0..n {
                let mut t = vec![];
                for j in i..n {
                    t.push(s[j]);
                }
                for j in 0..i {
                    t.push(s[j]);
                }
                if t < res {
                    res = t;
                }
            }
            res.into_iter().collect()
        }
    }
}

#[test]
fn test() {
    let s = "cba".to_string();
    let k = 1;
    let res = "acb".to_string();
    assert_eq!(Solution::orderly_queue(s, k), res);
    let s = "baaca".to_string();
    let k = 3;
    let res = "aaabc".to_string();
    assert_eq!(Solution::orderly_queue(s, k), res);
}
