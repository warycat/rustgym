struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, target: i32, path: &mut Vec<i32>, all: &mut Vec<Vec<i32>>);
}

impl Preorder for TreeLink {
    fn preorder(&self, target: i32, path: &mut Vec<i32>, all: &mut Vec<Vec<i32>>) {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            path.push(val);
            if node.left.is_none() && node.right.is_none() {
                if val == target {
                    all.push(path.to_vec());
                }
            } else {
                node.left.preorder(target - val, path, all);
                node.right.preorder(target - val, path, all);
            }
            path.pop();
        }
    }
}

impl Solution {
    fn path_sum(root: TreeLink, sum: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        root.preorder(sum, &mut path, &mut res);
        res
    }
}

#[test]
fn test() {
    let root = tree!(
        5,
        tree!(4, tree!(11, tree!(7), tree!(2)), None),
        tree!(8, tree!(13), tree!(4, tree!(5), tree!(1)))
    );
    let sum = 22;
    let res = vec_vec_i32![[5, 4, 11, 2], [5, 8, 4, 5]];
    assert_eq!(Solution::path_sum(root, sum), res);
}
