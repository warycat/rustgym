struct Solution;

impl Solution {
    fn can_transform(start: String, end: String) -> bool {
        let mut iter_a = start.chars();
        let mut iter_b = end.chars();
        let mut count_a = 0;
        let mut count_b = 0;
        while let (Some(mut a), Some(mut b)) = (iter_a.next(), iter_b.next()) {
            while a == 'X' {
                count_a += 1;
                a = if let Some(c) = iter_a.next() { c } else { 'Y' };
            }
            while b == 'X' {
                count_b += 1;
                b = if let Some(c) = iter_b.next() { c } else { 'Y' };
            }
            if a != b {
                return false;
            }
            if a == 'R' && count_a > count_b {
                return false;
            }
            if a == 'L' && count_a < count_b {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let start = "RXXLRXRXL".to_string();
    let end = "XRLXXRRLX".to_string();
    let res = true;
    assert_eq!(Solution::can_transform(start, end), res);
    let start = "XXRXXLXXXX".to_string();
    let end = "XXXXRXXLXX".to_string();
    let res = false;
    assert_eq!(Solution::can_transform(start, end), res);
}
