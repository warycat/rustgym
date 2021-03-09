struct Solution;

impl Solution {
    fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut indegree = vec![0; n];
        for e in edges {
            let to = e[1] as usize;
            indegree[to] += 1;
        }
        let mut res = vec![];
        for i in 0..n {
            if indegree[i] == 0 {
                res.push(i as i32);
            }
        }
        res
    }
}

#[test]
fn test() {
    let n = 6;
    let edges = vec_vec_i32![[0, 1], [0, 2], [2, 5], [3, 4], [4, 2]];
    let res = vec![0, 3];
    assert_eq!(Solution::find_smallest_set_of_vertices(n, edges), res);
    let n = 5;
    let edges = vec_vec_i32![[0, 1], [2, 1], [3, 1], [1, 4], [2, 4]];
    let res = vec![0, 2, 3];
    assert_eq!(Solution::find_smallest_set_of_vertices(n, edges), res);
}
