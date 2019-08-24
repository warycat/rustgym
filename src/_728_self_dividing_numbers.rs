struct Solution;

impl Solution {
    fn is_self_dividing(x: i32) -> bool {
        let mut n = x;
        while n > 0 {
            let last = n % 10;
            if last == 0 {
                return false;
            } else {
                if x % last != 0 {
                    return false;
                }
                n /= 10;
            }
        }
        true
    }

    fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        for i in left..=right {
            if Self::is_self_dividing(i) {
                res.push(i);
            }
        }
        res
    }
}

#[test]
fn test() {
    let left = 1;
    let right = 22;
    let res = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22];
    assert_eq!(Solution::self_dividing_numbers(left, right), res);
}
