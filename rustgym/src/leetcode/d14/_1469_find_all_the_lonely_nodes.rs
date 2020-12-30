struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, nodes: &mut Vec<i32>, is_lonely: bool);
}

impl Preorder for TreeLink {
    fn preorder(&self, nodes: &mut Vec<i32>, is_lonely: bool) {
        if let Some(node) = self {
            let node = node.borrow();
            if is_lonely {
                nodes.push(node.val);
            }
            let mut count = 0;
            if node.left.is_some() {
                count += 1;
            }
            if node.right.is_some() {
                count += 1;
            }
            node.left.preorder(nodes, count == 1);
            node.right.preorder(nodes, count == 1);
        }
    }
}

impl Solution {
    fn get_lonely_nodes(root: TreeLink) -> Vec<i32> {
        let mut res = vec![];
        root.preorder(&mut res, false);
        res
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2, None, tree!(4)), tree!(3));
    let res = vec![4];
    assert_eq!(Solution::get_lonely_nodes(root), res);
    let root = tree!(
        7,
        tree!(1, tree!(6), None),
        tree!(4, tree!(5), tree!(3, None, tree!(2)))
    );
    let res = vec![6, 2];
    assert_eq!(Solution::get_lonely_nodes(root), res);
    let root = tree!(
        11,
        tree!(99, tree!(77, tree!(55, tree!(33), None), None), None),
        tree!(88, None, tree!(66, None, tree!(44, None, tree!(22))))
    );
    let res = vec![77, 55, 33, 66, 44, 22];
    assert_eq!(Solution::get_lonely_nodes(root), res);
    let root = tree!(197);
    let res: Vec<i32> = vec![];
    assert_eq!(Solution::get_lonely_nodes(root), res);
}
