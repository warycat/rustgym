struct Solution;
use std::collections::HashMap;

impl Solution {
    fn delete_tree_nodes(nodes: i32, parent: Vec<i32>, value: Vec<i32>) -> i32 {
        let mut hm: HashMap<usize, Vec<usize>> = HashMap::new();
        let n = nodes as usize;
        let mut root = n;
        for i in 0..n {
            if parent[i] != -1 {
                hm.entry(parent[i] as usize).or_default().push(i);
            } else {
                root = i;
            }
        }
        let (_, size) = Self::postorder(root, &hm, &value, n);
        size as i32
    }

    fn postorder(
        i: usize,
        hm: &HashMap<usize, Vec<usize>>,
        value: &[i32],
        n: usize,
    ) -> (i32, usize) {
        let mut sum = value[i];
        let mut size = 1;
        if let Some(children) = hm.get(&i) {
            for &j in children {
                let child = Self::postorder(j, hm, value, n);
                sum += child.0;
                size += child.1;
            }
        }
        if sum == 0 {
            (0, 0)
        } else {
            (sum, size)
        }
    }
}

#[test]
fn test() {
    let nodes = 7;
    let parent = vec![-1, 0, 0, 1, 2, 2, 2];
    let value = vec![1, -2, 4, 0, -2, -1, -1];
    let res = 2;
    assert_eq!(Solution::delete_tree_nodes(nodes, parent, value), res);
}
