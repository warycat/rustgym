struct Solution;

impl Solution {
    fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        Self::dp(1, 0, n, 0, &nums)
    }

    fn dp(player: i32, start: usize, end: usize, sum: i32, nums: &[i32]) -> bool {
        if start == end {
            if player > 0 {
                sum >= 0
            } else {
                sum < 0
            }
        } else {
            let mut res = false;
            res |= !Self::dp(-player, start + 1, end, sum + player * nums[start], nums);
            if !res {
                res |= !Self::dp(-player, start, end - 1, sum + player * nums[end - 1], nums);
            }
            res
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 5, 2];
    let res = false;
    assert_eq!(Solution::predict_the_winner(nums), res);
    let nums = vec![1, 5, 233, 7];
    let res = true;
    assert_eq!(Solution::predict_the_winner(nums), res);
}
