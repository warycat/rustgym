struct Solution;

impl Solution {
    fn find_substring_in_wrapround_string(p: String) -> i32 {
        let mut count: [usize; 26] = [0; 26];
        let p: Vec<u8> = p.bytes().collect();
        let n = p.len();
        for i in 0..n {
            let j = (p[i] - b'a') as usize;
            count[j] = 1;
        }
        let mut l = 1;
        for i in 1..n {
            let j = (p[i] - b'a') as usize;
            let k = (p[i - 1] - b'a') as usize;
            l = if (k + 1) % 26 == j { l + 1 } else { 1 };
            count[j] = count[j].max(l);
        }
        let res: usize = count.iter().sum();
        res as i32
    }
}

#[test]
fn test() {
    let p = "a".to_string();
    let res = 1;
    assert_eq!(Solution::find_substring_in_wrapround_string(p), res);
    let p = "cac".to_string();
    let res = 2;
    assert_eq!(Solution::find_substring_in_wrapround_string(p), res);
    let p = "zab".to_string();
    let res = 6;
    assert_eq!(Solution::find_substring_in_wrapround_string(p), res);
}
