struct Solution;

use std::collections::VecDeque;

impl Solution {
    fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];
        let n = num_courses as usize;
        let mut edges: Vec<Vec<usize>> = vec![vec![]; n];
        let mut indegrees: Vec<usize> = vec![0; n];
        let mut queue: VecDeque<usize> = VecDeque::new();
        for e in prerequisites {
            let u = e[1] as usize;
            let v = e[0] as usize;
            edges[u].push(v);
            indegrees[v] += 1;
        }
        for u in 0..n {
            if indegrees[u] == 0 {
                queue.push_back(u);
            }
        }
        while let Some(u) = queue.pop_front() {
            res.push(u);
            for &v in &edges[u] {
                indegrees[v] -= 1;
                if indegrees[v] == 0 {
                    queue.push_back(v);
                }
            }
        }
        if res.len() == n {
            res.into_iter().map(|v| v as i32).collect()
        } else {
            vec![]
        }
    }
}

#[test]
fn test() {
    let num_courses = 2;
    let prerequisites: Vec<Vec<i32>> = vec_vec_i32![[1, 0]];
    let res = vec![0, 1];
    assert_eq!(Solution::find_order(num_courses, prerequisites), res);
    let num_courses = 4;
    let prerequisites: Vec<Vec<i32>> = vec_vec_i32![[1, 0], [2, 0], [3, 1], [3, 2]];
    let res = vec![0, 1, 2, 3];
    assert_eq!(Solution::find_order(num_courses, prerequisites), res);
}
