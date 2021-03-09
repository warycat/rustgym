struct Solution;

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let mut visited: HashSet<Vec<i32>> = HashSet::new();
        let next = vec![
            vec![1, 3],
            vec![0, 4, 2],
            vec![1, 5],
            vec![0, 4],
            vec![3, 1, 5],
            vec![2, 4],
        ];
        let solved = vec![1, 2, 3, 4, 5, 0];
        let mut queue: VecDeque<(Vec<i32>, usize, i32)> = VecDeque::new();
        let mut line = vec![];
        board
            .into_iter()
            .for_each(|v| v.into_iter().for_each(|x| line.push(x)));
        let zero = line.iter().position(|&x| x == 0).unwrap();
        visited.insert(line.to_vec());
        queue.push_back((line, zero, 0));
        while let Some((line, zero, count)) = queue.pop_front() {
            if line == solved {
                return count;
            }
            for &index in &next[zero] {
                let mut copy = line.to_vec();
                copy.swap(index, zero);
                if visited.insert(copy.to_vec()) {
                    queue.push_back((copy, index, count + 1));
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    let board = vec_vec_i32![[1, 2, 3], [4, 0, 5]];
    let res = 1;
    assert_eq!(Solution::sliding_puzzle(board), res);
    let board = vec_vec_i32![[1, 2, 3], [5, 4, 0]];
    let res = -1;
    assert_eq!(Solution::sliding_puzzle(board), res);
    let board = vec_vec_i32![[4, 1, 2], [5, 0, 3]];
    let res = 5;
    assert_eq!(Solution::sliding_puzzle(board), res);
    let board = vec_vec_i32![[3, 2, 4], [1, 5, 0]];
    let res = 14;
    assert_eq!(Solution::sliding_puzzle(board), res);
}
