struct Solution;
use rustgym_util::*;

trait Postorder {
    fn from_vec(preorder: &[i32], inorder: &[i32]) -> Self;
}

impl Postorder for TreeLink {
    fn from_vec(preorder: &[i32], inorder: &[i32]) -> Self {
        let n = preorder.len();
        if n == 0 {
            None
        } else {
            if n == 1 {
                tree!(preorder[0])
            } else {
                let i = inorder.binary_search(&preorder[0]).unwrap();
                tree!(
                    preorder[0],
                    TreeLink::from_vec(&preorder[1..=i], &inorder[0..i]),
                    TreeLink::from_vec(&preorder[i + 1..], &inorder[i + 1..])
                )
            }
        }
    }
}

impl Solution {
    fn bst_from_preorder(preorder: Vec<i32>) -> TreeLink {
        let mut inorder: Vec<i32> = preorder.clone();
        inorder.sort_unstable();
        TreeLink::from_vec(&preorder, &inorder)
    }
}

#[test]
fn test() {
    let preorder = vec![8, 5, 1, 7, 10, 12];
    let res = tree!(8, tree!(5, tree!(1), tree!(7)), tree!(10, None, tree!(12)));
    assert_eq!(Solution::bst_from_preorder(preorder), res);
}
