struct Solution;

impl Solution {
    fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut temp = vec![0; n];
        Self::merge_sort(0, n, &mut nums, &mut temp) as i32
    }

    fn merge_sort(start: usize, end: usize, nums: &mut Vec<i32>, temp: &mut Vec<i32>) -> usize {
        if start + 1 >= end {
            return 0;
        }
        let mid = start + (end - start) / 2;
        let mut res = 0;
        res += Self::merge_sort(start, mid, nums, temp);
        res += Self::merge_sort(mid, end, nums, temp);
        let mut i = start;
        let mut j = mid;
        while i < mid {
            while j < end && nums[i] as i64 > 2 * nums[j] as i64 {
                j += 1;
            }
            res += j - mid;
            i += 1;
        }
        let mut k = start;
        let mut i = start;
        let mut j = mid;
        while i < mid || j < end {
            if i == mid {
                temp[k] = nums[j];
                k += 1;
                j += 1;
                continue;
            }
            if j == end {
                temp[k] = nums[i];
                k += 1;
                i += 1;
                continue;
            }
            if nums[i] < nums[j] {
                temp[k] = nums[i];
                i += 1;
            } else {
                temp[k] = nums[j];
                j += 1;
            }
            k += 1;
        }
        nums[start..end].clone_from_slice(&temp[start..end]);
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 3, 2, 3, 1];
    let res = 2;
    assert_eq!(Solution::reverse_pairs(nums), res);
    let nums = vec![2, 4, 3, 5, 1];
    let res = 3;
    assert_eq!(Solution::reverse_pairs(nums), res);
    let nums = vec![
        2147483647, 2147483647, 2147483647, 2147483647, 2147483647, 2147483647,
    ];
    let res = 0;
    assert_eq!(Solution::reverse_pairs(nums), res);
}
