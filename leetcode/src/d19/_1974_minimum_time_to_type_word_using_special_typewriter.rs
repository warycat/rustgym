struct Solution;

impl Solution {
    fn min_time_to_type(word: String) -> i32 {
        let mut current = b'a';
        let mut res = 0;
        for b in word.bytes() {
            if b == current {
                res += 1;
            } else {
                let d = if b > current {
                    b - current
                } else {
                    current - b
                };
                let d = d.min(26 - d);
                current = b;
                res += d as i32 + 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let word = "abc".to_string();
    let res = 5;
    assert_eq!(Solution::min_time_to_type(word), res);
    let word = "bza".to_string();
    let res = 7;
    assert_eq!(Solution::min_time_to_type(word), res);
    let word = "zjpc".to_string();
    let res = 34;
    assert_eq!(Solution::min_time_to_type(word), res);
}
