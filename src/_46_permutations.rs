struct Solution;

impl Solution {
    fn permute_r(arr: &mut Vec<i32>, start: usize, length: usize, res: &mut Vec<Vec<i32>>) {
        if start == length {
            res.push(arr.to_vec());
            return;
        }
        for i in start..length {
            arr.swap(start, i);
            Self::permute_r(arr, start + 1, length, res);
            arr.swap(start, i);
        }
    }

    fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let n = nums.len();
        Self::permute_r(&mut nums, 0, n, &mut res);
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    let mut ans: Vec<Vec<i32>> = [
        [1, 2, 3],
        [1, 3, 2],
        [2, 1, 3],
        [2, 3, 1],
        [3, 1, 2],
        [3, 2, 1],
    ]
    .iter()
    .map(|v| v.to_vec())
    .collect();
    let mut res = Solution::permute(nums);
    ans.sort_unstable();
    res.sort_unstable();
    assert_eq!(res, ans);
}
