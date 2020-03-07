struct Solution;

impl Solution {
    fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = nums.len();
        let mut visited = vec![false; n];
        for i in 0..n {
            if visited[i] {
                continue;
            }
            let mut j = i;
            let mut length = 0;
            while !visited[j] {
                visited[j] = true;
                j = nums[j] as usize;
                length += 1;
            }
            res = res.max(length);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![5, 4, 0, 3, 1, 6, 2];
    let res = 4;
    assert_eq!(Solution::array_nesting(nums), res);
}
