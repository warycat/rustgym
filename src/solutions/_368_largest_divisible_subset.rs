struct Solution;

impl Solution {
    fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut index: Vec<usize> = (0..n).collect();
        let mut size: Vec<usize> = vec![1; n];
        let mut max_size = 1;
        nums.sort_unstable();
        for i in 0..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && size[j] + 1 > size[i] {
                    index[i] = j;
                    size[i] = size[j] + 1;
                    max_size = max_size.max(size[i]);
                }
            }
        }
        let mut res = vec![];
        for i in 0..n {
            if size[i] == max_size {
                let mut j = i;
                while index[j] != j {
                    res.push(nums[j]);
                    j = index[j];
                }
                res.push(nums[j]);
                break;
            }
        }
        res.reverse();
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    let res = vec![1, 2];
    assert_eq!(Solution::largest_divisible_subset(nums), res);
    let nums = vec![1, 2, 4, 8];
    let res = vec![1, 2, 4, 8];
    assert_eq!(Solution::largest_divisible_subset(nums), res);
    let nums = vec![4, 8, 10, 240];
    let res = vec![4, 8, 240];
    assert_eq!(Solution::largest_divisible_subset(nums), res);
}
