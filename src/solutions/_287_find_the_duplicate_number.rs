struct Solution;

impl Solution {
    fn find_duplicate(nums: Vec<i32>) -> i32 {
        let n = (nums.len() - 1) as i32;
        let mut low = 1;
        let mut high = n;
        while low < high {
            let mid = low + (high - low) / 2;
            let mut count = 0;
            for &x in &nums {
                if x <= mid {
                    count += 1;
                }
            }
            if count <= mid {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low
    }
}

#[test]
fn test() {
    let nums = vec![1, 3, 4, 2, 2];
    let res = 2;
    assert_eq!(Solution::find_duplicate(nums), res);
    let nums = vec![3, 1, 3, 4, 2];
    let res = 3;
    assert_eq!(Solution::find_duplicate(nums), res);
}
