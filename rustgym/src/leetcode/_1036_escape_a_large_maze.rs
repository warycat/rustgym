struct Solution;

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        Self::bfs(&blocked, source.clone(), target.clone()) && Self::bfs(&blocked, target, source)
    }

    fn bfs(blocked: &[Vec<i32>], source: Vec<i32>, target: Vec<i32>) -> bool {
        let mut visited: HashSet<Vec<i32>> = HashSet::new();
        let mut queue: VecDeque<Vec<i32>> = VecDeque::new();
        let n = 1_000_000;
        for point in blocked {
            visited.insert(point.clone());
        }
        queue.push_back(source.clone());
        visited.insert(source);
        let mut step = 0;
        while let Some(point) = queue.pop_front() {
            step += 1;
            if point == target || step == 20000 {
                return true;
            }
            let i = point[0];
            let j = point[1];
            let up = vec![i - 1, j];
            let left = vec![i, j - 1];
            let down = vec![i + 1, j];
            let right = vec![i, j + 1];
            if i > 0 && visited.insert(up.clone()) {
                queue.push_back(up);
            }
            if j > 0 && visited.insert(left.clone()) {
                queue.push_back(left);
            }
            if i + 1 < n && visited.insert(down.clone()) {
                queue.push_back(down);
            }
            if j + 1 < n && visited.insert(right.clone()) {
                queue.push_back(right);
            }
        }
        false
    }
}

#[test]
fn test() {
    let blocked = vec_vec_i32![[0, 1], [1, 0]];
    let source = vec![0, 0];
    let target = vec![0, 2];
    let res = false;
    assert_eq!(Solution::is_escape_possible(blocked, source, target), res);
    let blocked = vec_vec_i32![];
    let source = vec![0, 0];
    let target = vec![999999, 999999];
    let res = true;
    assert_eq!(Solution::is_escape_possible(blocked, source, target), res);
}
