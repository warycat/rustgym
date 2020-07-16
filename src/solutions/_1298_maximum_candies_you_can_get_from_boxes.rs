struct Solution;

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    fn max_candies(
        mut status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut opened_boxes: VecDeque<usize> = VecDeque::new();
        let mut closed_boxes: HashSet<usize> = HashSet::new();
        let mut keys_available: VecDeque<usize> = VecDeque::new();
        let mut res = 0;
        for x in initial_boxes {
            let x = x as usize;
            if status[x] == 0 {
                closed_boxes.insert(x);
            } else {
                opened_boxes.push_back(x);
            }
        }
        loop {
            let mut done = true;
            for _ in 0..opened_boxes.len() {
                if let Some(x) = opened_boxes.pop_front() {
                    status[x] = 2;
                    res += candies[x];
                    for &k in keys[x].iter() {
                        done = false;
                        let k = k as usize;
                        keys_available.push_back(k);
                    }
                    for &y in contained_boxes[x].iter() {
                        done = false;
                        let y = y as usize;
                        if status[y] == 0 {
                            closed_boxes.insert(y);
                        } else {
                            opened_boxes.push_back(y);
                        }
                    }
                }
            }
            for _ in 0..keys_available.len() {
                if let Some(k) = keys_available.pop_front() {
                    if status[k] == 2 {
                        continue;
                    }
                    if closed_boxes.remove(&k) {
                        done = false;
                        opened_boxes.push_back(k);
                    } else {
                        keys_available.push_back(k);
                    }
                }
            }
            if done {
                break;
            }
        }
        res
    }
}

#[test]
fn test() {
    let status = vec![1, 0, 1, 0];
    let candies = vec![7, 5, 4, 100];
    let keys = vec_vec_i32![[], [], [1], []];
    let contained_boxes = vec_vec_i32![[1, 2], [3], [], []];
    let initial_boxes = vec![0];
    let res = 16;
    assert_eq!(
        Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes),
        res
    );
    let status = vec![1, 0, 0, 0, 0, 0];
    let candies = vec![1, 1, 1, 1, 1, 1];
    let keys = vec_vec_i32![[1, 2, 3, 4, 5], [], [], [], [], []];
    let contained_boxes = vec_vec_i32![[1, 2, 3, 4, 5], [], [], [], [], []];
    let initial_boxes = vec![0];
    let res = 6;
    assert_eq!(
        Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes),
        res
    );
    let status = vec![1, 1, 1];
    let candies = vec![100, 1, 100];
    let keys = vec_vec_i32![[], [0, 2], []];
    let contained_boxes = vec_vec_i32![[], [], []];
    let initial_boxes = vec![1];
    let res = 1;
    assert_eq!(
        Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes),
        res
    );
    let status = vec![1];
    let candies = vec![100];
    let keys = vec_vec_i32![[]];
    let contained_boxes = vec_vec_i32![[]];
    let initial_boxes = vec![];
    let res = 0;
    assert_eq!(
        Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes),
        res
    );
    let status = vec![1, 1, 1];
    let candies = vec![2, 3, 2];
    let keys = vec_vec_i32![[], [], []];
    let contained_boxes = vec_vec_i32![[], [], []];
    let initial_boxes = vec![2, 1, 0];
    let res = 7;
    assert_eq!(
        Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes),
        res
    );
}
