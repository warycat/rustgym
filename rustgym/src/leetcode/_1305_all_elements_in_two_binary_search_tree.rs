struct Solution;
use rustgym_util::*;

trait Inorder {
    fn inorder(&self, v: &mut Vec<i32>);
}

impl Inorder for TreeLink {
    fn inorder(&self, v: &mut Vec<i32>) {
        if let Some(node) = self {
            let val = node.borrow().val;
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            left.inorder(v);
            v.push(val);
            right.inorder(v);
        }
    }
}

impl Solution {
    fn get_all_elements(root1: TreeLink, root2: TreeLink) -> Vec<i32> {
        let mut res = vec![];
        let mut v1 = vec![];
        let mut v2 = vec![];
        root1.inorder(&mut v1);
        root2.inorder(&mut v2);
        let mut i = 0;
        let mut j = 0;
        let n = v1.len();
        let m = v2.len();
        while i < n || j < m {
            if i == n {
                res.push(v2[j]);
                j += 1;
                continue;
            }
            if j == m {
                res.push(v1[i]);
                i += 1;
                continue;
            }
            if v1[i] < v2[j] {
                res.push(v1[i]);
                i += 1;
            } else {
                res.push(v2[j]);
                j += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let root1 = tree!(2, tree!(1), tree!(4));
    let root2 = tree!(1, tree!(0), tree!(3));
    let res = vec![0, 1, 1, 2, 3, 4];
    assert_eq!(Solution::get_all_elements(root1, root2), res);
    let root1 = tree!(0, tree!(-10), tree!(10));
    let root2 = tree!(5, tree!(1, tree!(0), tree!(2)), tree!(7));
    let res = vec![-10, 0, 0, 1, 2, 5, 7, 10];
    assert_eq!(Solution::get_all_elements(root1, root2), res);
    let root1 = None;
    let root2 = tree!(5, tree!(1, tree!(0), tree!(2)), tree!(7));
    let res = vec![0, 1, 2, 5, 7];
    assert_eq!(Solution::get_all_elements(root1, root2), res);
    let root1 = tree!(0, tree!(-10), tree!(10));
    let root2 = None;
    let res = vec![-10, 0, 10];
    assert_eq!(Solution::get_all_elements(root1, root2), res);
}
