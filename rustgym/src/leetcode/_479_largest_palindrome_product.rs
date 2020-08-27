struct Solution;

impl Solution {
    fn largest_palindrome(n: i32) -> i32 {
        if n == 1 {
            return 9;
        }
        let max = 10u64.pow(n as u32) - 1;
        for i in (0..max).rev() {
            let left: String = i.to_string();
            let right: String = i.to_string().chars().rev().collect();
            let palindrome = format!("{}{}", left, right);
            if let Ok(value) = palindrome.parse::<u64>() {
                let mut j = max;
                while j * j >= value {
                    if value % j == 0 {
                        return (value % 1337) as i32;
                    }
                    j -= 1;
                }
            }
        }
        0
    }
}

#[test]
fn test() {
    let n = 2;
    let res = 987;
    assert_eq!(Solution::largest_palindrome(n), res);
    let n = 1;
    let res = 9;
    assert_eq!(Solution::largest_palindrome(n), res);
    let n = 5;
    let res = 677;
    assert_eq!(Solution::largest_palindrome(n), res);
}
