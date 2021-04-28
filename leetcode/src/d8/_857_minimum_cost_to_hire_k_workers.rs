struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let n = quality.len();
        let mut workers: Vec<usize> = (0..n).collect();
        let rate: Vec<f64> = (0..n).map(|i| wage[i] as f64 / quality[i] as f64).collect();
        workers.sort_by(|&i, &j| rate[i].partial_cmp(&rate[j]).unwrap());
        let mut res = std::f64::MAX;
        let mut queue: BinaryHeap<i32> = BinaryHeap::new();
        let mut sum = 0;
        for i in workers {
            let r = rate[i];
            let q = quality[i];
            queue.push(q);
            sum += q;
            if queue.len() > k {
                sum -= queue.pop().unwrap();
            }
            if queue.len() == k {
                res = res.min(r * sum as f64);
            }
        }
        res
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let quality = vec![10, 20, 5];
    let wage = vec![70, 50, 30];
    let k = 2;
    let res = 105.00000;
    assert_approx_eq!(Solution::mincost_to_hire_workers(quality, wage, k), res);
    let quality = vec![3, 1, 10, 10, 1];
    let wage = vec![4, 8, 2, 2, 7];
    let k = 3;
    let res = 30.666667;
    assert_approx_eq!(Solution::mincost_to_hire_workers(quality, wage, k), res);
}
