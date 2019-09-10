struct Solution;

impl Solution {
    fn convert_to_title(mut n: i32) -> String {
        let mut v: Vec<char> = vec![];
        while n > 0 {
            let x = ((n - 1) % 26) as u8;
            let c = (x + b'A') as char;
            v.insert(0, c);
            n = (n - 1) / 26;
        }
        v.iter().collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::convert_to_title(701), "ZY".to_string());
}
