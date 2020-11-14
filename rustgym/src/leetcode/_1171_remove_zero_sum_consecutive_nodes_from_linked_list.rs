struct Solution;
use rustgym_util::*;

use std::collections::HashMap;

impl Solution {
    fn remove_zero_sum_sublists(mut head: ListLink) -> ListLink {
        let mut stack: Vec<i32> = vec![];
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let mut sum = 0;
        hm.insert(0, 0);
        while let Some(node) = head {
            let val = node.val;
            head = node.next;
            if val == 0 {
                continue;
            }
            sum += val;
            if let Some(&size) = hm.get(&sum) {
                sum -= val;
                while stack.len() > size {
                    let top = stack.pop().unwrap();
                    hm.remove(&sum);
                    sum -= top;
                }
            } else {
                stack.push(val);
                hm.insert(sum, stack.len());
            }
        }
        let mut prev = None;
        while let Some(top) = stack.pop() {
            prev = ListLink::link(top, prev);
        }
        prev
    }
}

#[test]
fn test() {
    let head = list!(1, 2, -3, 3, 1);
    let res = list!(3, 1);
    assert_eq!(Solution::remove_zero_sum_sublists(head), res);
    let head = list!(1, 2, 3, -3, 4);
    let res = list!(1, 2, 4);
    assert_eq!(Solution::remove_zero_sum_sublists(head), res);
    let head = list!(1, 2, 3, -3, -2);
    let res = list!(1);
    assert_eq!(Solution::remove_zero_sum_sublists(head), res);
}
