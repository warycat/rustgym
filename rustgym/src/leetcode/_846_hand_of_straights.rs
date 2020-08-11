struct Solution;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::iter::FromIterator;

impl Solution {
    fn is_n_straight_hand(hand: Vec<i32>, w: i32) -> bool {
        let mut btm: BTreeMap<i32, usize> = BTreeMap::new();
        for card in hand {
            *btm.entry(card).or_default() += 1;
        }
        let w = w as usize;
        let mut queue: VecDeque<(i32, usize)> = VecDeque::from_iter(btm);
        while !queue.is_empty() {
            let first = queue.pop_front().unwrap();
            let mut stack: Vec<(i32, usize)> = vec![];
            for i in 1..w {
                if let Some(front) = queue.pop_front() {
                    if front.0 != first.0 + i as i32 || front.1 < first.1 {
                        return false;
                    } else {
                        let left = front.1 - first.1;
                        if left > 0 {
                            stack.push((front.0, left));
                        }
                    }
                } else {
                    return false;
                }
            }
            while let Some(last) = stack.pop() {
                queue.push_front(last);
            }
        }
        true
    }
}

#[test]
fn test() {
    let hand = vec![1, 2, 3, 6, 2, 3, 4, 7, 8];
    let w = 3;
    let res = true;
    assert_eq!(Solution::is_n_straight_hand(hand, w), res);
    let hand = vec![1, 2, 3, 4, 5];
    let w = 4;
    let res = false;
    assert_eq!(Solution::is_n_straight_hand(hand, w), res);
    let hand = vec![5, 1];
    let w = 2;
    let res = false;
    assert_eq!(Solution::is_n_straight_hand(hand, w), res);
}
