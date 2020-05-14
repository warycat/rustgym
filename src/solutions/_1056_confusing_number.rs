struct Solution;

impl Solution {
    fn rotate(d: i32) -> Option<i32> {
        match d {
            0 => Some(0),
            1 => Some(1),
            6 => Some(9),
            8 => Some(8),
            9 => Some(6),
            _ => None,
        }
    }

    fn confusing_number(mut n: i32) -> bool {
        let num = n;
        let mut rev = 0;
        while n > 0 {
            let d = n % 10;
            if let Some(t) = Self::rotate(d) {
                rev *= 10;
                rev += t;
                n /= 10;
            } else {
                return false;
            }
        }
        rev != num
    }
}

#[test]
fn test() {
    assert_eq!(Solution::confusing_number(6), true);
    assert_eq!(Solution::confusing_number(89), true);
    assert_eq!(Solution::confusing_number(11), false);
    assert_eq!(Solution::confusing_number(25), false);
}
