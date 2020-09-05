struct Solution;

use std::cmp::Ordering::*;
use std::collections::VecDeque;

impl Solution {
    fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if buildings.is_empty() {
            return vec![];
        }
        let mut queue: VecDeque<Vec<Vec<i32>>> = buildings
            .into_iter()
            .map(|x| vec![vec![x[0], x[2]], vec![x[1], 0]])
            .collect();
        while queue.len() > 1 {
            let a = queue.pop_front().unwrap();
            let b = queue.pop_front().unwrap();
            let c = Self::merge(a, b);
            queue.push_back(c);
        }
        queue.pop_front().unwrap()
    }

    fn merge(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut i = 0;
        let mut j = 0;
        let mut res = vec![];
        let mut prev_h = 0;
        let mut l = 0;
        let mut r = 0;
        let mut x;
        while i < a.len() && j < b.len() {
            match a[i][0].cmp(&b[j][0]) {
                Equal => {
                    x = a[i][0];
                    l = a[i][1];
                    r = b[j][1];
                    i += 1;
                    j += 1;
                }
                Less => {
                    x = a[i][0];
                    l = a[i][1];
                    i += 1;
                }
                Greater => {
                    x = b[j][0];
                    r = b[j][1];
                    j += 1;
                }
            }
            let h = l.max(r);
            if h != prev_h {
                res.push(vec![x, h]);
                prev_h = h;
            }
        }
        while i < a.len() {
            let x = a[i][0];
            let h = a[i][1];
            i += 1;
            if h != prev_h {
                res.push(vec![x, h]);
                prev_h = h;
            }
        }
        while j < b.len() {
            let x = b[j][0];
            let h = b[j][1];
            j += 1;
            if h != prev_h {
                res.push(vec![x, h]);
                prev_h = h;
            }
        }
        res
    }
}

#[test]
fn test() {
    let buildings = vec_vec_i32![
        [2, 9, 10],
        [3, 7, 15],
        [5, 12, 12],
        [15, 20, 10],
        [19, 24, 8]
    ];
    let res = vec![
        [2, 10],
        [3, 15],
        [7, 12],
        [12, 0],
        [15, 10],
        [20, 8],
        [24, 0],
    ];
    assert_eq!(Solution::get_skyline(buildings), res);
}
