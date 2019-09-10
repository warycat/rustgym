struct Solution;

impl Solution {
    fn convert_to_base7(mut num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let mut base_7: Vec<char> = vec![];
        let minus: bool = num < 0;
        if num < 0 {
            num = -num;
        }
        while num > 0 {
            let c = ((num % 7) as u8 + b'0') as char;
            base_7.push(c);
            num /= 7;
        }
        if minus {
            base_7.push('-');
        }
        base_7.reverse();
        let res: String = base_7.iter().collect();
        res
    }
}

#[test]
fn test() {
    let num = 100;
    let res = "202".to_string();
    assert_eq!(Solution::convert_to_base7(num), res);
    let num = -7;
    let res = "-10".to_string();
    assert_eq!(Solution::convert_to_base7(num), res);
    let num = -999;
    let res = "-2625".to_string();
    assert_eq!(Solution::convert_to_base7(num), res);
}
