struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(self, all: &mut usize, distance: i32) -> [usize; 10];
}

impl Postorder for TreeLink {
    fn postorder(self, all: &mut usize, distance: i32) -> [usize; 10] {
        if let Some(node) = self {
            let mut res = [0; 10];
            let mut node = node.borrow_mut();
            let mut left = node.left.take();
            let mut right = node.right.take();

            if let (None, None) = (left.as_mut(), right.as_mut()) {
                res[0] = 1;
            } else {
                let l = left.postorder(all, distance);
                let r = right.postorder(all, distance);
                for i in 0..9 {
                    for j in 0..9 {
                        if i + j <= (distance - 2) as usize {
                            *all += l[i] * r[j];
                        }
                    }
                }
                for i in 0..9 {
                    res[i + 1] += l[i];
                }
                for i in 0..9 {
                    res[i + 1] += r[i];
                }
            }
            res
        } else {
            [0; 10]
        }
    }
}

impl Solution {
    fn count_pairs(root: TreeLink, distance: i32) -> i32 {
        let mut res = 0;
        if distance < 2 {
            return 0;
        }
        root.postorder(&mut res, distance);
        res as i32
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2, None, tree!(4)), tree!(3));
    let distance = 3;
    let res = 1;
    assert_eq!(Solution::count_pairs(root, distance), res);
    let root = tree!(
        1,
        tree!(2, tree!(4), tree!(5)),
        tree!(3, tree!(6), tree!(7))
    );
    let distance = 3;
    let res = 2;
    assert_eq!(Solution::count_pairs(root, distance), res);
}
