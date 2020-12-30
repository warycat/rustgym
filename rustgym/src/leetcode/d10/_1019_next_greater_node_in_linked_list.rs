struct Solution;
use rustgym_util::*;

impl Solution {
    fn next_larger_nodes(mut head: ListLink) -> Vec<i32> {
        let mut nodes = vec![];
        while let Some(node) = head {
            nodes.push(node.val);
            head = node.next;
        }
        let n = nodes.len();
        let mut stack: Vec<usize> = vec![];
        let mut res = vec![0; n];
        for i in 0..n {
            while let Some(j) = stack.pop() {
                if nodes[j] < nodes[i] {
                    res[j] = nodes[i];
                } else {
                    stack.push(j);
                    break;
                }
            }
            stack.push(i);
        }
        res
    }
}

#[test]
fn test() {
    let head = list!(2, 1, 5);
    let res = vec![5, 5, 0];
    assert_eq!(Solution::next_larger_nodes(head), res);
    let head = list!(2, 7, 4, 3, 5);
    let res = vec![7, 0, 5, 5, 0];
    assert_eq!(Solution::next_larger_nodes(head), res);
    let head = list!(1, 7, 5, 1, 9, 2, 5, 1);
    let res = vec![7, 9, 9, 9, 0, 5, 0, 0];
    assert_eq!(Solution::next_larger_nodes(head), res);
}
