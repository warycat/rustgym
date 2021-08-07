struct Solution;

use std::cmp::Ordering;
use std::collections::BTreeMap;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(v: &[i32]) -> Self {
        Point { x: v[0], y: v[1] }
    }
    fn manhattan(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(PartialEq, Eq)]
struct Pair {
    i: usize,
    j: usize,
}

impl Ord for Pair {
    fn cmp(&self, other: &Pair) -> Ordering {
        let res = other.i.cmp(&self.i);
        if let Ordering::Equal = res {
            other.j.cmp(&self.j)
        } else {
            res
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Pair) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> Vec<i32> {
        let mut btm: BTreeMap<i32, Vec<Pair>> = BTreeMap::new();
        let n = workers.len();
        let mut res: Vec<i32> = vec![0; n];
        let m = bikes.len();
        for i in 0..n {
            for j in 0..m {
                let worker = Point::new(&workers[i]);
                let bike = Point::new(&bikes[j]);
                let distance = worker.manhattan(&bike);
                btm.entry(distance).or_default().push(Pair { i, j });
            }
        }
        let mut workers: Vec<bool> = vec![true; n];
        let mut bikes: Vec<bool> = vec![true; m];
        for (_, mut pairs) in btm.into_iter() {
            pairs.sort_unstable();
            while let Some(p) = pairs.pop() {
                if workers[p.i] && bikes[p.j] {
                    let i = p.i;
                    let j = p.j;
                    workers[i] = false;
                    bikes[j] = false;
                    res[i] = j as i32;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let workers: Vec<Vec<i32>> = vec_vec_i32![[0, 0], [2, 1]];
    let bikes: Vec<Vec<i32>> = vec_vec_i32![[1, 2], [3, 3]];
    let res = vec![1, 0];
    assert_eq!(Solution::assign_bikes(workers, bikes), res);
    let workers: Vec<Vec<i32>> = vec_vec_i32![[0, 0], [1, 1], [2, 0]];
    let bikes: Vec<Vec<i32>> = vec_vec_i32![[1, 0], [2, 2], [2, 1]];
    let res = vec![0, 2, 1];
    assert_eq!(Solution::assign_bikes(workers, bikes), res);
}
