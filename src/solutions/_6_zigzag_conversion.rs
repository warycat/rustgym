struct Solution;

impl Solution {
    fn convert(s: String, num_rows: i32) -> String {
        let mut res = "".to_string();
        let n = num_rows as usize;
        if n == 1 {
            return s;
        }
        let m = s.len();
        let mut v: Vec<String> = vec!["".to_string(); n];
        let mut row = 0;
        let mut direction = true;
        for j in 0..m {
            v[row] += &s[j..=j];
            if row == 0 {
                direction = true;
                row += 1;
            } else if row == n - 1 {
                direction = false;
                row -= 1;
            } else {
                if direction {
                    row += 1;
                } else {
                    row -= 1;
                }
            }
        }
        for t in v {
            res += &t;
        }
        res
    }
}

#[test]
fn test() {
    let s = "PAYPALISHIRING".to_string();
    let num_rows = 3;
    let res = "PAHNAPLSIIGYIR".to_string();
    assert_eq!(Solution::convert(s, num_rows), res);
    let s = "PAYPALISHIRING".to_string();
    let num_rows = 4;
    let res = "PINALSIGYAHRPI".to_string();
    assert_eq!(Solution::convert(s, num_rows), res);
    let s = "AB".to_string();
    let num_rows = 1;
    let res = "AB".to_string();
    assert_eq!(Solution::convert(s, num_rows), res);
}
