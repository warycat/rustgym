struct Solution;
use rustgym_util::*;

impl Solution {
    fn construct_from_pre_post(pre: Vec<i32>, post: Vec<i32>) -> TreeLink {
        Self::build(&mut 0, &mut 0, &pre, &post)
    }

    fn build(i: &mut usize, j: &mut usize, pre: &[i32], post: &[i32]) -> TreeLink {
        let val = pre[*i];
        *i += 1;
        let mut left = None;
        let mut right = None;
        if val != post[*j] {
            left = Self::build(i, j, pre, post);
        }
        if val != post[*j] {
            right = Self::build(i, j, pre, post);
        }
        *j += 1;
        tree!(val, left, right)
    }
}

#[test]
fn test() {
    let pre = vec![1, 2, 4, 5, 3, 6, 7];
    let post = vec![4, 5, 2, 6, 7, 3, 1];
    let res = tree!(
        1,
        tree!(2, tree!(4), tree!(5)),
        tree!(3, tree!(6), tree!(7))
    );
    assert_eq!(Solution::construct_from_pre_post(pre, post), res);
}
