struct Solution;

impl Solution {
    fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        let m = 1 << k;
        let mut set = vec![false; m];
        let mut x = 0;
        let n = s.len();
        let mask = m as u32 - 1;
        for (i, c) in s.char_indices().rev() {
            x <<= 1;
            x |= (c as u8 - b'0') as u32;
            x &= mask;
            if i + k <= n {
                set[x as usize] = true;
            }
        }
        set.iter().all(|&b| b)
    }
}

#[test]
fn test() {
    let s = "00110110".to_string();
    let k = 2;
    let res = true;
    assert_eq!(Solution::has_all_codes(s, k), res);
    let s = "00110".to_string();
    let k = 2;
    let res = true;
    assert_eq!(Solution::has_all_codes(s, k), res);
    let s = "0110".to_string();
    let k = 1;
    let res = true;
    assert_eq!(Solution::has_all_codes(s, k), res);
    let s = "0110".to_string();
    let k = 2;
    let res = false;
    assert_eq!(Solution::has_all_codes(s, k), res);
    let s = "0000000001011100".to_string();
    let k = 4;
    let res = false;
    assert_eq!(Solution::has_all_codes(s, k), res);
}
