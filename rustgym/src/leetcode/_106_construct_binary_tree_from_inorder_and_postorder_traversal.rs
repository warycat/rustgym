struct Solution;
use rustgym_util::*;

impl Solution {
    fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> TreeLink {
        let n = inorder.len();
        Self::build(&inorder[0..n], &postorder[0..n])
    }

    fn build(inorder: &[i32], postorder: &[i32]) -> TreeLink {
        let n = inorder.len();
        if n == 0 {
            None
        } else {
            let val = postorder[n - 1];
            let i = inorder.iter().position(|&x| x == val).unwrap();
            tree!(
                val,
                Self::build(&inorder[0..i], &postorder[0..i]),
                Self::build(&inorder[i + 1..n], &postorder[i..n - 1])
            )
        }
    }
}

#[test]
fn test() {
    let inorder = vec![9, 3, 15, 20, 7];
    let postorder = vec![9, 15, 7, 20, 3];
    let res = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    assert_eq!(Solution::build_tree(inorder, postorder), res);
}
