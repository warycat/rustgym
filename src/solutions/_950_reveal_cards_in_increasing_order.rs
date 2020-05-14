struct Solution;
use std::collections::VecDeque;

impl Solution {
    fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort_unstable();
        let mut queue: VecDeque<i32> = VecDeque::new();
        let n = deck.len();
        for i in (0..n).rev() {
            let last = deck[i];
            if let Some(bottom) = queue.pop_back() {
                queue.push_front(bottom);
            }
            queue.push_front(last);
        }
        queue.into_iter().collect()
    }
}

#[test]
fn test() {
    let deck = vec![17, 13, 11, 2, 3, 5, 7];
    let res = vec![2, 13, 3, 11, 5, 17, 7];
    assert_eq!(Solution::deck_revealed_increasing(deck), res);
}
