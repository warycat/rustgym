struct Solution;

impl Solution {
    fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let n = nums.len();
        if n < 3 {
            return res;
        }
        nums.sort_unstable();
        for i in 0..n - 2 {
            let a = nums[i];
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut j = i + 1;
            let mut k = n - 1;
            while j < k {
                let b = nums[j];
                let c = nums[k];
                let sum = a + b + c;
                if sum == 0 {
                    res.push(vec![a, b, c]);
                    j += 1;
                    k -= 1;
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                    while j < k && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                } else {
                    if sum < 0 {
                        j += 1;
                    } else {
                        k -= 1;
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let res: Vec<Vec<i32>> = vec_vec_i32![[-1, -1, 2], [-1, 0, 1]];
    assert_eq!(Solution::three_sum(nums), res);
    let nums = vec![-2, 0, 1, 1, 2];
    let res: Vec<Vec<i32>> = vec_vec_i32![[-2, 0, 2], [-2, 1, 1]];
    assert_eq!(Solution::three_sum(nums), res);
}
