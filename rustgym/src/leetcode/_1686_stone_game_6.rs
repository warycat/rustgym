struct Solution;
use std::cmp::Ordering::*;

impl Solution {
    fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let n = alice_values.len();
        let mut sums = vec![];
        for i in 0..n {
            sums.push((
                alice_values[i] + bob_values[i],
                alice_values[i],
                bob_values[i],
            ));
        }
        sums.sort_unstable();
        let mut sum_a = 0;
        let mut sum_b = 0;
        let mut turn = 0;
        while let Some((_, a, b)) = sums.pop() {
            if turn % 2 == 0 {
                sum_a += a;
            } else {
                sum_b += b;
            }
            turn += 1;
        }
        match sum_a.cmp(&sum_b) {
            Greater => 1,
            Less => -1,
            Equal => 0,
        }
    }
}

#[test]
fn test() {
    let alice_values = vec![1, 3];
    let bob_values = vec![2, 1];
    let res = 1;
    assert_eq!(Solution::stone_game_vi(alice_values, bob_values), res);
    let alice_values = vec![1, 2];
    let bob_values = vec![3, 1];
    let res = 0;
    assert_eq!(Solution::stone_game_vi(alice_values, bob_values), res);
    let alice_values = vec![2, 4, 3];
    let bob_values = vec![1, 6, 7];
    let res = -1;
    assert_eq!(Solution::stone_game_vi(alice_values, bob_values), res);
}
