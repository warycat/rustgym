struct Solution;
use rustgym_util::*;

impl Solution {
    fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> TreeLink {
        let n = preorder.len();
        Self::build(&preorder[0..n], &inorder[0..n])
    }

    fn build(preorder: &[i32], inorder: &[i32]) -> TreeLink {
        let n = preorder.len();
        if n == 0 {
            None
        } else {
            let val = preorder[0];
            let i = inorder.iter().position(|&x| x == val).unwrap();
            tree!(
                val,
                Self::build(&preorder[1..=i], &inorder[0..i]),
                Self::build(&preorder[i + 1..], &inorder[i + 1..])
            )
        }
    }
}

#[test]
fn test() {
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    let res = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    assert_eq!(Solution::build_tree(preorder, inorder), res);
}
