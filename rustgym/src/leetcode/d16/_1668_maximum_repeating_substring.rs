struct Solution;

impl Solution {
    fn max_repeating(sequence: String, word: String) -> i32 {
        let n = sequence.len();
        let m = word.len();
        let mut k = n / m;
        while k > 0 {
            let mut s = "".to_string();
            for _ in 0..k {
                s += &word;
            }
            if sequence.contains(&s) {
                break;
            }
            k -= 1;
        }
        k as i32
    }
}

#[test]
fn test() {
    let sequence = "ababc".to_string();
    let word = "ab".to_string();
    let res = 2;
    assert_eq!(Solution::max_repeating(sequence, word), res);
    let sequence = "ababc".to_string();
    let word = "ba".to_string();
    let res = 1;
    assert_eq!(Solution::max_repeating(sequence, word), res);
    let sequence = "ababc".to_string();
    let word = "ac".to_string();
    let res = 0;
    assert_eq!(Solution::max_repeating(sequence, word), res);
}
