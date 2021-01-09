struct Solution;

impl Solution {
    fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut len: Vec<usize> = vec![1; n];
        let mut cnt: Vec<usize> = vec![1; n];
        for i in 0..n {
            for j in 0..i {
                if nums[j] < nums[i] {
                    if len[j] + 1 == len[i] {
                        cnt[i] += cnt[j];
                    }
                    if len[j] == len[i] {
                        len[i] += 1;
                        cnt[i] = cnt[j];
                    }
                }
            }
        }
        let max_len = *len.iter().max().unwrap();
        let mut res = 0;
        for i in 0..n {
            if len[i] == max_len {
                res += cnt[i];
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 3, 5, 4, 7];
    let res = 2;
    assert_eq!(Solution::find_number_of_lis(nums), res);
    let nums = vec![2, 2, 2, 2, 2];
    let res = 5;
    assert_eq!(Solution::find_number_of_lis(nums), res);
}
