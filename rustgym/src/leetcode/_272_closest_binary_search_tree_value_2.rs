struct Solution;
use rustgym_util::*;

trait Inorder {
    fn inorder(&self, values: &mut Vec<i32>);
}

impl Inorder for TreeLink {
    fn inorder(&self, values: &mut Vec<i32>) {
        if let Some(node) = self {
            let node = node.borrow();
            node.left.inorder(values);
            values.push(node.val);
            node.right.inorder(values);
        }
    }
}

impl Solution {
    fn closest_k_values(root: TreeLink, target: f64, k: i32) -> Vec<i32> {
        let mut values = vec![];
        root.inorder(&mut values);
        let n = values.len();
        let k = k as usize;
        let mut start = 0;
        let mut end = n;
        while end - start > k {
            if (values[start] as f64 - target).abs() > (values[end - 1] as f64 - target).abs() {
                start += 1;
            } else {
                end -= 1;
            }
        }
        values[start..end].to_vec()
    }
}

#[test]
fn test() {
    let root = tree!(4, tree!(2, tree!(1), tree!(3)), tree!(5));
    let target = 3.714286;
    let k = 2;
    let res = vec![3, 4];
    assert_eq!(Solution::closest_k_values(root, target, k), res);
}
