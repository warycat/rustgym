struct Solution;

impl Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        let mut m = 0;
        let mut prev: Option<(i32, usize)> = None;
        for i in 0..n {
            if let Some((value, count)) = prev {
                if value == nums[i] {
                    if count == 1 {
                        prev = Some((nums[i], 2));
                        nums[m] = nums[i];
                        m += 1;
                    }
                } else {
                    prev = Some((nums[i], 1));
                    nums[m] = nums[i];
                    m += 1;
                }
            } else {
                prev = Some((nums[i], 1));
                nums[m] = nums[i];
                m += 1;
            }
        }
        m as i32
    }
}

#[test]
fn test() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    let res = 5;
    assert_eq!(Solution::remove_duplicates(&mut nums), res);
}
