struct Solution;

impl Solution {
    fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let n = n as usize;
        let mut indegree = vec![0; n];
        let mut outdegree = vec![0; n];
        let mut edge = 0;
        for i in 0..n {
            if left_child[i] != -1 {
                edge += 1;
                outdegree[i] += 1;
                indegree[left_child[i] as usize] += 1;
            }
            if right_child[i] != -1 {
                edge += 1;
                outdegree[i] += 1;
                indegree[right_child[i] as usize] += 1;
            }
        }
        let degree = (0..n).any(|i| {
            let a = n != 1 && indegree[i] == 0 && outdegree[i] == 0;
            let b = indegree[i] > 1;
            let c = outdegree[i] > 2;
            a || b || c
        });
        !degree && edge + 1 == n
    }
}

#[test]
fn test() {
    let n = 4;
    let left_child = vec![1, -1, 3, -1];
    let right_child = vec![2, -1, -1, -1];
    let res = true;
    assert_eq!(
        Solution::validate_binary_tree_nodes(n, left_child, right_child),
        res
    );
    let n = 4;
    let left_child = vec![1, -1, 3, -1];
    let right_child = vec![2, 3, -1, -1];
    let res = false;
    assert_eq!(
        Solution::validate_binary_tree_nodes(n, left_child, right_child),
        res
    );
    let n = 2;
    let left_child = vec![1, 0];
    let right_child = vec![-1, -1];
    let res = false;
    assert_eq!(
        Solution::validate_binary_tree_nodes(n, left_child, right_child),
        res
    );
    let n = 6;
    let left_child = vec![1, -1, -1, 4, -1, -1];
    let right_child = vec![2, -1, -1, 5, -1, -1];
    let res = false;
    assert_eq!(
        Solution::validate_binary_tree_nodes(n, left_child, right_child),
        res
    );
}
