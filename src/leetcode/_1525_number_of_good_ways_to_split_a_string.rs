struct Solution;

impl Solution {
    fn num_splits(s: String) -> i32 {
        let mut total = vec![0; 26];
        for b in s.bytes() {
            total[(b - b'a') as usize] += 1;
        }

        let mut left = vec![0; 26];
        let mut right = total.to_vec();
        let mut res = 0;
        for b in s.bytes() {
            left[(b - b'a') as usize] += 1;
            right[(b - b'a') as usize] -= 1;
            let mut diff = 0;
            for i in 0..26 {
                if left[i] > 0 {
                    diff += 1;
                }
                if right[i] > 0 {
                    diff -= 1;
                }
            }
            if diff == 0 {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "aacaba".to_string();
    let res = 2;
    assert_eq!(Solution::num_splits(s), res);
    let s = "abcd".to_string();
    let res = 1;
    assert_eq!(Solution::num_splits(s), res);
    let s = "aaaaa".to_string();
    let res = 4;
    assert_eq!(Solution::num_splits(s), res);
    let s = "acbadbaada".to_string();
    let res = 2;
    assert_eq!(Solution::num_splits(s), res);
}
