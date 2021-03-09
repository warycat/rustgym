struct Solution;

impl Solution {
    fn max_diff(num: i32) -> i32 {
        let mut min = std::i32::MAX;
        let mut max = std::i32::MIN;
        for i in 0..10 {
            for j in 0..10 {
                let mut s: Vec<u8> = num.to_string().bytes().collect();
                let n = s.len();
                for k in 0..n {
                    if s[k] == i + b'0' {
                        s[k] = j + b'0';
                    }
                }
                if s[0] == b'0' {
                    continue;
                }
                let x = s
                    .into_iter()
                    .map(|b| b as char)
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                if x == 0 {
                    continue;
                }
                min = min.min(x);
                max = max.max(x);
            }
        }
        max - min
    }
}

#[test]
fn test() {
    let num = 555;
    let res = 888;
    assert_eq!(Solution::max_diff(num), res);
    let num = 9;
    let res = 8;
    assert_eq!(Solution::max_diff(num), res);
    let num = 123456;
    let res = 820000;
    assert_eq!(Solution::max_diff(num), res);
    let num = 10000;
    let res = 80000;
    assert_eq!(Solution::max_diff(num), res);
    let num = 9288;
    let res = 8700;
    assert_eq!(Solution::max_diff(num), res);
}
