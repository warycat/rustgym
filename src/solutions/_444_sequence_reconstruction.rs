struct Solution;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    fn sequence_reconstruction(org: Vec<i32>, seqs: Vec<Vec<i32>>) -> bool {
        let n = org.len();
        let mut indegree = vec![0; n];
        let mut marked = vec![false; n];
        let mut graph = vec![HashSet::new(); n];
        for seq in seqs {
            let m = seq.len();
            for i in 0..m {
                let u = (seq[i] - 1) as usize;
                if u >= n {
                    return false;
                }
                marked[u] = true;
                if i + 1 < m {
                    let v = (seq[i + 1] - 1) as usize;
                    if v >= n {
                        return false;
                    }
                    if v == u {
                        return false;
                    }
                    if graph[u].insert(v) {
                        indegree[v] += 1;
                    }
                }
            }
        }
        if marked.iter().any(|x| !x) {
            return false;
        }
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut stack = vec![];
        for i in 0..n {
            if indegree[i] == 0 {
                queue.push_back(i);
            }
        }
        while !queue.is_empty() {
            if queue.len() > 1 {
                return false;
            }
            if let Some(u) = queue.pop_front() {
                stack.push((u + 1) as i32);
                for &v in &graph[u] {
                    indegree[v] -= 1;
                    if indegree[v] == 0 {
                        queue.push_back(v);
                    }
                }
            }
        }
        stack == org
    }
}

#[test]
fn test() {
    let org = vec![1, 2, 3];
    let seqs = vec_vec_i32![[1, 2], [1, 3]];
    let res = false;
    assert_eq!(Solution::sequence_reconstruction(org, seqs), res);
    let org = vec![1, 2, 3];
    let seqs = vec_vec_i32![[1, 2]];
    let res = false;
    assert_eq!(Solution::sequence_reconstruction(org, seqs), res);
    let org = vec![1, 2, 3];
    let seqs = vec_vec_i32![[1, 2], [1, 3], [2, 3]];
    let res = true;
    assert_eq!(Solution::sequence_reconstruction(org, seqs), res);
    let org = vec![4, 1, 5, 2, 6, 3];
    let seqs = vec_vec_i32![[5, 2, 6, 3], [4, 1, 5, 2]];
    let res = true;
    assert_eq!(Solution::sequence_reconstruction(org, seqs), res);
    let org = vec![1];
    let seqs = vec_vec_i32![];
    let res = false;
    assert_eq!(Solution::sequence_reconstruction(org, seqs), res);
    let org = vec![1];
    let seqs = vec_vec_i32![[1, 1]];
    let res = false;
    assert_eq!(Solution::sequence_reconstruction(org, seqs), res);
    let org = vec![1, 2];
    let seqs = vec_vec_i32![[1, 2], [2, 1]];
    let res = false;
    assert_eq!(Solution::sequence_reconstruction(org, seqs), res);
    let org = vec![3, 2, 1];
    let seqs = vec_vec_i32![[1, 2], [1, 3], [2, 3]];
    let res = false;
    assert_eq!(Solution::sequence_reconstruction(org, seqs), res);
    let org = vec![1];
    let seqs = vec_vec_i32![[2]];
    let res = false;
    assert_eq!(Solution::sequence_reconstruction(org, seqs), res);
}
