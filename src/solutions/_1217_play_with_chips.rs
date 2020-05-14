struct Solution;

impl Solution {
    fn min_cost_to_move_chips(chips: Vec<i32>) -> i32 {
        let n = chips.len();
        let mut a = 0;
        let mut b = 0;
        for i in 0..n {
            if chips[i] % 2 == 0 {
                a += 1;
            } else {
                b += 1;
            }
        }
        i32::min(a, b)
    }
}

#[test]
fn test() {
    let chips = vec![1, 2, 3];
    assert_eq!(Solution::min_cost_to_move_chips(chips), 1);
    let chips = vec![2, 2, 2, 3, 3];
    assert_eq!(Solution::min_cost_to_move_chips(chips), 2);
}
