struct Solution;

impl Solution {
    fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let sums: Vec<i32> = nums.windows(k).map(|w| w.iter().sum()).collect();
        let n = sums.len();
        let mut left = vec![];
        let mut left_max = 0;
        let mut left_index = 0;
        for i in 0..n {
            if sums[i] > left_max {
                left_max = sums[i];
                left_index = i;
            }
            left.push((left_max, left_index));
        }

        let mut right = vec![];
        let mut right_max = 0;
        let mut right_index = n;
        for i in (0..n).rev() {
            if sums[i] >= right_max {
                right_max = sums[i];
                right_index = i;
            }
            right.push((right_max, right_index));
        }
        right.reverse();
        let mut mid_max = 0;
        let mut res = vec![0, 0, 0];
        for i in k..n - k {
            let sum_3 = sums[i] + left[i - k].0 + right[i + k].0;
            if sum_3 > mid_max {
                mid_max = sum_3;
                res = vec![left[i - k].1, i, right[i + k].1];
            }
        }
        res.into_iter().map(|x| x as i32).collect()
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 1, 2, 6, 7, 5, 1];
    let k = 2;
    let res = vec![0, 3, 5];
    assert_eq!(Solution::max_sum_of_three_subarrays(nums, k), res);
}
