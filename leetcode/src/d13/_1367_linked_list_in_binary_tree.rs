struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, cur: &mut Vec<i32>, found: &mut bool, path: &[i32]);
}

impl Preorder for TreeLink {
    fn preorder(&self, cur: &mut Vec<i32>, found: &mut bool, path: &[i32]) {
        if let Some(node) = self {
            let node = node.borrow();
            cur.push(node.val);
            if cur.ends_with(path) {
                *found = true;
            }
            node.left.preorder(cur, found, path);
            node.right.preorder(cur, found, path);
            cur.pop();
        }
    }
}

impl Solution {
    fn is_sub_path(mut head: ListLink, root: TreeLink) -> bool {
        let mut path = vec![];
        while let Some(node) = head {
            path.push(node.val);
            head = node.next;
        }
        let mut cur = vec![];
        let mut res = false;
        root.preorder(&mut cur, &mut res, &path);
        res
    }
}

#[test]
fn test() {
    let head = list!(4, 2, 8);
    let root = tree!(
        1,
        tree!(4, None, tree!(2, tree!(1), None)),
        tree!(4, tree!(2, tree!(6), tree!(8, tree!(1), tree!(3))), None)
    );
    let res = true;
    assert_eq!(Solution::is_sub_path(head, root), res);
}
