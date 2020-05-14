struct Solution;

impl Solution {
    fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut num: Option<i32> = None;
        for x in nums {
            if let Some(y) = num {
                if x == y {
                    count += 1;
                } else {
                    count -= 1;
                    if count == 0 {
                        num = None;
                    }
                }
            } else {
                num = Some(x);
                count += 1;
            }
        }
        num.unwrap()
    }
}

#[test]
fn test() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    assert_eq!(Solution::majority_element(nums), 2);
    let nums = vec![3, 3, 4];
    assert_eq!(Solution::majority_element(nums), 3);
}
