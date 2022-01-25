struct Solution;

#[derive(Clone)]
struct Pair {
    value: i32,
    index: usize,
}

impl Solution {
    fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut pairs: Vec<Pair> = vec![];
        for i in 0..n {
            pairs.push(Pair {
                value: nums[i],
                index: i,
            });
        }
        pairs.sort_by_key(|p| p.value);
        let k = k as usize;
        let mut k_pairs: Vec<Pair> = vec![];
        for i in 0..k {
            k_pairs.push(pairs[n - k + i].clone());
        }
        k_pairs.sort_by_key(|p| p.index);
        let mut res = vec![];
        for i in 0..k {
            res.push(k_pairs[i].value);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![2, 1, 3, 3];
    let k = 2;
    let res = vec![3, 3];
    assert_eq!(Solution::max_subsequence(nums, k), res);
    let nums = vec![-1, -2, 3, 4];
    let k = 3;
    let res = vec![-1, 3, 4];
    assert_eq!(Solution::max_subsequence(nums, k), res);
    let nums = vec![3, 4, 3, 3];
    let k = 2;
    let res = vec![4, 3];
    assert_eq!(Solution::max_subsequence(nums, k), res);
}
