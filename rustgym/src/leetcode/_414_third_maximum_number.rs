struct Solution;
use std::cmp::Ordering::*;

impl Solution {
    fn third_max(nums: Vec<i32>) -> i32 {
        let mut max1 = nums[0];
        let mut m2: Option<i32> = None;
        let mut m3: Option<i32> = None;
        for x in nums {
            match x.cmp(&max1) {
                Greater => {
                    m3 = m2;
                    m2 = Some(max1);
                    max1 = x;
                }
                Less => {
                    if let Some(max2) = m2 {
                        match x.cmp(&max2) {
                            Greater => {
                                m3 = m2;
                                m2 = Some(x);
                            }
                            Less => {
                                if let Some(max3) = m3 {
                                    if x > max3 {
                                        m3 = Some(x);
                                    }
                                } else {
                                    m3 = Some(x);
                                }
                            }
                            Equal => {}
                        }
                    } else {
                        m2 = Some(x);
                    }
                }
                Equal => {}
            }
        }
        if let Some(max3) = m3 {
            max3
        } else {
            max1
        }
    }
}

#[test]
fn test() {
    let nums = vec![3, 2, 1];
    assert_eq!(Solution::third_max(nums), 1);
    let nums = vec![1, 2];
    assert_eq!(Solution::third_max(nums), 2);
    let nums = vec![2, 2, 3, 1];
    assert_eq!(Solution::third_max(nums), 1);
    let nums = vec![1, 2, 2, 5, 3, 5];
    assert_eq!(Solution::third_max(nums), 2);
}
