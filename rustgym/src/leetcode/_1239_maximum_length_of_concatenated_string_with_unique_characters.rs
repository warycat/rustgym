struct Solution;

impl Solution {
    fn max_length(arr: Vec<String>) -> i32 {
        let arr: Vec<u32> = arr
            .into_iter()
            .filter_map(|s| {
                let mut bitset = 0;
                for b in s.bytes() {
                    let bit = 1 << (b - b'a');
                    if bitset & bit != 0 {
                        return None;
                    } else {
                        bitset |= bit
                    }
                }
                Some(bitset)
            })
            .collect();
        let n = arr.len();
        let mut res = 0;
        Self::dfs(0, 0, &mut res, &arr, n);
        res as i32
    }

    fn dfs(start: usize, cur: u32, max: &mut u32, arr: &[u32], n: usize) {
        if start == n {
            *max = (*max).max(cur.count_ones());
        } else {
            if arr[start] & cur == 0 {
                Self::dfs(start + 1, cur | arr[start], max, arr, n);
            }
            Self::dfs(start + 1, cur, max, arr, n);
        }
    }
}

#[test]
fn test() {
    let arr = vec_string!["un", "iq", "ue"];
    let res = 4;
    assert_eq!(Solution::max_length(arr), res);
    let arr = vec_string!["cha", "r", "act", "ers"];
    let res = 6;
    assert_eq!(Solution::max_length(arr), res);
    let arr = vec_string!["abcdefghijklmnopqrstuvwxyz"];
    let res = 26;
    assert_eq!(Solution::max_length(arr), res);
}
