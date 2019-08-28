struct Solution;

impl Solution {
    fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut b5 = 0;
        let mut b10 = 0;
        for b in bills {
            match b {
                5 => {
                    b5 += 1;
                }
                10 => {
                    if b5 > 0 {
                        b10 += 1;
                        b5 -= 1;
                    } else {
                        return false;
                    }
                }
                20 => {
                    if b10 > 0 && b5 > 0 {
                        b10 -= 1;
                        b5 -= 1;
                    } else if b5 >= 3 {
                        b5 -= 3;
                    } else {
                        return false;
                    }
                }
                _ => {}
            }
        }
        true
    }
}

#[test]
fn test() {
    let bills = vec![5, 5, 5, 10, 20];
    assert_eq!(Solution::lemonade_change(bills), true);
    let bills = vec![5, 5, 10];
    assert_eq!(Solution::lemonade_change(bills), true);
    let bills = vec![10, 10];
    assert_eq!(Solution::lemonade_change(bills), false);
    let bills = vec![5, 5, 10, 10, 20];
    assert_eq!(Solution::lemonade_change(bills), false);
    let bills = vec![
        5, 5, 10, 20, 5, 5, 5, 5, 5, 5, 5, 5, 5, 10, 5, 5, 20, 5, 20, 5,
    ];
    assert_eq!(Solution::lemonade_change(bills), true);
}
