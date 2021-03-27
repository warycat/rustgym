struct Solution;

impl Solution {
    fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let n = nums.len();
        let mut w = vec![];
        for i in 0..k {
            w.push(nums[i]);
        }
        w.sort_unstable();
        let mut res = vec![Self::median(&w, k)];
        for i in k..n {
            let pos = w.binary_search(&nums[i - k]).unwrap_or_else(|e| e);
            w.remove(pos);
            let pos = w.binary_search(&nums[i]).unwrap_or_else(|e| e);
            w.insert(pos, nums[i]);
            res.push(Self::median(&w, k));
        }
        res
    }

    fn median(v: &[i32], k: usize) -> f64 {
        (v[(k - 1) / 2] as f64 + v[k / 2] as f64) / 2.0
    }
}

#[test]
fn test() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let res = vec![1.0, -1.0, -1.0, 3.0, 5.0, 6.0];
    assert_eq!(Solution::median_sliding_window(nums, k), res);
}
