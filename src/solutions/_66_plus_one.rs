struct Solution;

impl Solution {
    fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut sum: Vec<i32> = vec![];
        let mut carry: i32 = 0;
        for (i, d) in digits.iter().rev().enumerate() {
            let x = if i == 0 { d + 1 } else { d + carry };
            carry = x / 10;
            sum.insert(0, x % 10);
        }
        if carry != 0 {
            sum.insert(0, carry);
        }
        sum
    }
}

#[test]
fn test() {
    assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0])
}
