struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, size: &mut usize, nodes: &mut Vec<i32>, voyage: &[i32]) -> bool;
}

impl Preorder for TreeLink {
    fn preorder(&self, size: &mut usize, nodes: &mut Vec<i32>, voyage: &[i32]) -> bool {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            if voyage[*size] != val {
                return false;
            }
            *size += 1;
            if node.left.is_none() && node.right.is_none() {
                return true;
            }
            if node.left.is_some() {
                if node.left.as_ref().unwrap().borrow().val == voyage[*size] {
                    node.left.preorder(size, nodes, voyage)
                        && node.right.preorder(size, nodes, voyage)
                } else {
                    nodes.push(val);
                    node.right.preorder(size, nodes, voyage)
                        && node.left.preorder(size, nodes, voyage)
                }
            } else {
                node.right.preorder(size, nodes, voyage)
            }
        } else {
            true
        }
    }
}

impl Solution {
    fn flip_match_voyage(root: TreeLink, voyage: Vec<i32>) -> Vec<i32> {
        let mut nodes = vec![];
        let mut size: usize = 0;
        if root.preorder(&mut size, &mut nodes, &voyage) {
            nodes
        } else {
            vec![-1]
        }
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2), None);
    let voyage = vec![2, 1];
    let res = vec![-1];
    assert_eq!(Solution::flip_match_voyage(root, voyage), res);
    let root = tree!(1, tree!(2), tree!(3));
    let voyage = vec![1, 3, 2];
    let res = vec![1];
    assert_eq!(Solution::flip_match_voyage(root, voyage), res);
    let root = tree!(1, tree!(2), tree!(3));
    let voyage = vec![1, 2, 3];
    let res: Vec<i32> = vec![];
    assert_eq!(Solution::flip_match_voyage(root, voyage), res);
}
