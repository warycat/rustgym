struct Solution;

impl Solution {
    fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut indegrees: Vec<i32> = vec![0; n];
        let mut edges: Vec<Vec<usize>> = vec![vec![]; n];
        let mut queue: Vec<usize> = vec![];
        let mut res: Vec<usize> = vec![];
        for e in prerequisites {
            let u = e[1] as usize;
            let v = e[0] as usize;
            indegrees[v] += 1;
            edges[u].push(v);
        }
        for u in 0..n {
            let indegree = indegrees[u];
            if indegree == 0 {
                queue.push(u);
            }
        }
        while let Some(u) = queue.pop() {
            res.push(u);
            while let Some(v) = edges[u].pop() {
                indegrees[v] -= 1;
                if indegrees[v] == 0 {
                    queue.push(v);
                }
            }
        }
        res.len() == n
    }
}

#[test]
fn test() {
    let num_courses = 2;
    let prerequisites: Vec<Vec<i32>> = vec_vec_i32![[1, 0]];
    let res = true;
    assert_eq!(Solution::can_finish(num_courses, prerequisites), res);
    let num_courses = 2;
    let prerequisites: Vec<Vec<i32>> = vec_vec_i32![[1, 0], [0, 1]];
    let res = false;
    assert_eq!(Solution::can_finish(num_courses, prerequisites), res);
}
