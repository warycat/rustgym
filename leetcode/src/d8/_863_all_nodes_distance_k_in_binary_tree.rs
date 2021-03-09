struct Solution;
use rustgym_util::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    fn distance_k(root: TreeLink, p: TreeLink, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
        Self::dfs(root, &mut adj);
        let mut res = vec![];
        let mut queue: VecDeque<(i32, usize)> = VecDeque::new();
        let start = p.unwrap().borrow().val;
        let mut visited: HashSet<i32> = HashSet::new();
        visited.insert(start);
        queue.push_back((start, 0));
        while let Some((u, d)) = queue.pop_front() {
            if d == k {
                res.push(u);
            } else {
                for &mut v in adj.entry(u).or_default() {
                    if visited.insert(v) {
                        queue.push_back((v, d + 1));
                    }
                }
            }
        }
        res
    }

    fn dfs(root: TreeLink, adj: &mut HashMap<i32, Vec<i32>>) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            let u = node.val;
            let left = node.left.take();
            let right = node.right.take();
            if left.is_some() {
                let v = left.as_ref().unwrap().borrow().val;
                adj.entry(u).or_default().push(v);
                adj.entry(v).or_default().push(u);
                Self::dfs(left, adj);
            }
            if right.is_some() {
                let v = right.as_ref().unwrap().borrow().val;
                adj.entry(u).or_default().push(v);
                adj.entry(v).or_default().push(u);
                Self::dfs(right, adj);
            }
        }
    }
}

#[test]
fn test() {
    let root = tree!(
        3,
        tree!(5, tree!(6), tree!(2, tree!(7), tree!(4))),
        tree!(1, tree!(0), tree!(8))
    );
    let p = tree!(5);
    let k = 2;
    let mut res = vec![7, 4, 1];
    let mut ans = Solution::distance_k(root, p, k);
    res.sort_unstable();
    ans.sort_unstable();
    assert_eq!(ans, res);
}
