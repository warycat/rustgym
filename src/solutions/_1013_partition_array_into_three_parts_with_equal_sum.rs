struct Solution;

impl Solution {
    fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let total: i32 = a.iter().sum();
        let mut parts = 0;
        let mut sum = 0;
        if total % 3 != 0 {
            return false;
        }
        for x in a {
            sum += x;
            if sum == total / 3 {
                parts += 1;
                if parts == 2 {
                    return true;
                }
                sum = 0;
            }
        }
        false
    }
}

#[test]
fn test() {
    let a = vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1];
    assert_eq!(Solution::can_three_parts_equal_sum(a), true);
    let a = vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1];
    assert_eq!(Solution::can_three_parts_equal_sum(a), false);
    let a = vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4];
    assert_eq!(Solution::can_three_parts_equal_sum(a), true);
}
