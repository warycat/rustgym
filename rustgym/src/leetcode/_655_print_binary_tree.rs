struct Solution;
use rustgym_util::*;

trait Postorder {
    fn height(&self) -> usize;
}

impl Postorder for TreeLink {
    fn height(&self) -> usize {
        if let Some(node) = self {
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            1 + left.height().max(right.height())
        } else {
            0
        }
    }
}

trait Preorder {
    fn print(&self, i: usize, j: usize, grid: &mut Vec<Vec<String>>, w: usize, h: usize);
}

impl Preorder for TreeLink {
    fn print(&self, i: usize, j: usize, grid: &mut Vec<Vec<String>>, w: usize, h: usize) {
        if let Some(node) = self {
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            let val = node.borrow().val;
            grid[i][j] = format!("{}", val);
            if i != h - 1 {
                let left_index = j - (w >> 2) - 1;
                let right_index = j + (w >> 2) + 1;
                left.print(i + 1, left_index, grid, w >> 1, h);
                right.print(i + 1, right_index, grid, w >> 1, h);
            }
        }
    }
}

impl Solution {
    fn print_tree(root: TreeLink) -> Vec<Vec<String>> {
        let h = root.height();
        let w = (1 << h) - 1;
        let mut res = vec![vec!["".to_string(); w]; h];
        root.print(0, w >> 1, &mut res, w, h);
        res
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2), None);
    let res = vec_vec_string![["", "1", ""], ["2", "", ""]];
    assert_eq!(Solution::print_tree(root), res);
    let root = tree!(1, tree!(2, None, tree!(4)), tree!(3));
    let res = vec_vec_string![
        ["", "", "", "1", "", "", ""],
        ["", "2", "", "", "", "3", ""],
        ["", "", "4", "", "", "", ""]
    ];
    assert_eq!(Solution::print_tree(root), res);
    let root = tree!(1, tree!(2, tree!(3, tree!(4), None), None), tree!(5));
    let res = vec_vec_string![
        ["", "", "", "", "", "", "", "1", "", "", "", "", "", "", ""],
        ["", "", "", "2", "", "", "", "", "", "", "", "5", "", "", ""],
        ["", "3", "", "", "", "", "", "", "", "", "", "", "", "", ""],
        ["4", "", "", "", "", "", "", "", "", "", "", "", "", "", ""]
    ];
    assert_eq!(Solution::print_tree(root), res);
}
