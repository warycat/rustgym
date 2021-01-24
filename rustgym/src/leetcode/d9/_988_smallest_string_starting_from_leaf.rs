struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, cur: &mut Vec<char>, min: &mut String);
}

impl Preorder for TreeLink {
    fn preorder(&self, cur: &mut Vec<char>, min: &mut String) {
        if let Some(node) = self {
            let node = node.borrow();
            let val = (node.val as u8 + b'a') as char;
            cur.push(val);
            if node.left.is_none() && node.right.is_none() {
                let s: String = cur.iter().rev().copied().collect();
                if min.is_empty() {
                    *min = s;
                } else {
                    if s < *min {
                        *min = s;
                    }
                }
            }
            node.left.preorder(cur, min);
            node.right.preorder(cur, min);
            cur.pop();
        }
    }
}

impl Solution {
    fn smallest_from_leaf(root: TreeLink) -> String {
        let mut cur: Vec<char> = vec![];
        let mut res: String = "".to_string();
        root.preorder(&mut cur, &mut res);
        res
    }
}

#[test]
fn test() {
    let root = tree!(
        0,
        tree!(1, tree!(3), tree!(4)),
        tree!(2, tree!(3), tree!(4))
    );
    let res = "dba".to_string();
    assert_eq!(Solution::smallest_from_leaf(root), res);
    let root = tree!(
        25,
        tree!(1, tree!(1), tree!(3)),
        tree!(3, tree!(0), tree!(2))
    );
    let res = "adz".to_string();
    assert_eq!(Solution::smallest_from_leaf(root), res);
    let root = tree!(
        2,
        tree!(2, None, tree!(1, tree!(0), None)),
        tree!(1, tree!(0), None)
    );
    let res = "abc".to_string();
    assert_eq!(Solution::smallest_from_leaf(root), res);
}
