struct Solution;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(PartialEq)]
struct State {
    id: usize,
    prob: f64,
}

impl State {
    fn new(id: usize, prob: f64) -> Self {
        State { id, prob }
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.prob.partial_cmp(&other.prob)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let n = n as usize;
        let start = start as usize;
        let end = end as usize;
        let mut adj: Vec<HashMap<usize, f64>> = vec![HashMap::new(); n];
        for (i, edge) in edges.into_iter().enumerate() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let p = succ_prob[i];
            adj[u].insert(v, p);
            adj[v].insert(u, p);
        }
        let mut probs: Vec<f64> = vec![0.0; n];
        let mut queue: BinaryHeap<State> = BinaryHeap::new();
        queue.push(State::new(start, 1.0));
        probs[start] = 1.0;
        while let Some(parent) = queue.pop() {
            if parent.id == end {
                return parent.prob;
            }
            for (&child_id, &prob) in &adj[parent.id] {
                let new_prob = parent.prob * prob;
                if new_prob > probs[child_id] {
                    probs[child_id] = new_prob;
                    queue.push(State::new(child_id, new_prob));
                }
            }
        }
        0.0
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let n = 3;
    let edges = vec_vec_i32![[0, 1], [1, 2], [0, 2]];
    let succ_prob = vec![0.5, 0.5, 0.2];
    let start = 0;
    let end = 2;
    let res = 0.25;
    assert_approx_eq!(
        Solution::max_probability(n, edges, succ_prob, start, end),
        res
    );

    let n = 3;
    let edges = vec_vec_i32![[0, 1], [1, 2], [0, 2]];
    let succ_prob = vec![0.5, 0.5, 0.3];
    let start = 0;
    let end = 2;
    let res = 0.3;
    assert_approx_eq!(
        Solution::max_probability(n, edges, succ_prob, start, end),
        res
    );

    let n = 3;
    let edges = vec_vec_i32![[0, 1]];
    let succ_prob = vec![0.5];
    let start = 0;
    let end = 2;
    let res = 0.00000;
    assert_approx_eq!(
        Solution::max_probability(n, edges, succ_prob, start, end),
        res
    );
}
