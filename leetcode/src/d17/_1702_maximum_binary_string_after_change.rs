struct Solution;

impl Solution {
    fn maximum_binary_string(binary: String) -> String {
        let binary: Vec<char> = binary.chars().collect();
        let mut res = "".to_string();
        let n = binary.len();
        let mut i = 0;
        while i < n && binary[i] == '1' {
            res.push('1');
            i += 1;
        }
        if i < n {
            let mut m = 0;
            let mut k = 0;
            while i < n {
                if binary[i] == '1' {
                    m += 1;
                }
                k += 1;
                i += 1;
            }
            for _ in 0..k - m - 1 {
                res.push('1');
            }
            res.push('0');
            for _ in 0..m {
                res.push('1');
            }
        }
        res
    }
}

#[test]
fn test() {
    let binary = "000110".to_string();
    let res = "111011".to_string();
    assert_eq!(Solution::maximum_binary_string(binary), res);
    let binary = "01".to_string();
    let res = "01".to_string();
    assert_eq!(Solution::maximum_binary_string(binary), res);
}
