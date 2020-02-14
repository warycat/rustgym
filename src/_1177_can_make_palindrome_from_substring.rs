struct Solution;

impl Solution {
    fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = s.len();
        let mut prefix: Vec<u32> = vec![0; n + 1];
        for (i, c) in s.char_indices() {
            prefix[i + 1] = prefix[i] ^ (1 << (c as u32 - 'a' as u32));
        }
        let mut res = vec![];
        for q in queries {
            let left = q[0] as usize;
            let right = q[1] as usize + 1;
            let k = q[2] as u32;
            res.push(k * 2 >= (prefix[right] ^ prefix[left]).count_ones() - 1);
        }
        res
    }
}

#[test]
fn test() {
    let s = "abcda".to_string();
    let queries = vec_vec_i32![[3, 3, 0], [1, 2, 0], [0, 3, 1], [0, 3, 2], [0, 4, 1]];
    let res = vec![true, false, false, true, true];
    assert_eq!(Solution::can_make_pali_queries(s, queries), res);
}
