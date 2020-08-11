struct Solution;

impl Solution {
    fn max_depth_after_split(seq: String) -> Vec<i32> {
        let n = seq.len();
        let mut level = 0;
        let mut res: Vec<i32> = vec![0; n];
        for (i, c) in seq.char_indices() {
            if c == '(' {
                res[i] = level % 2;
                level += 1;
            } else {
                level -= 1;
                res[i] = level % 2;
            }
        }
        res
    }
}

#[test]
fn test() {
    let seq = "(()())".to_string();
    let res = vec![0, 1, 1, 1, 1, 0];
    assert_eq!(Solution::max_depth_after_split(seq), res);
    let seq = "()(())()".to_string();
    let res = vec![0, 0, 0, 1, 1, 0, 0, 0];
    assert_eq!(Solution::max_depth_after_split(seq), res);
}
