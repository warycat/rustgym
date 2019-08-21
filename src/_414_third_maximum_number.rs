struct Solution;

impl Solution {
    fn third_max(nums: Vec<i32>) -> i32 {
        let mut max1 = nums[0];
        let mut m2: Option<i32> = None;
        let mut m3: Option<i32> = None;
        for x in nums {
            if x > max1 {
                m3 = m2;
                m2 = Some(max1);
                max1 = x;
            } else if x < max1 {
                if let Some(max2) = m2 {
                    if x > max2 {
                        m3 = m2;
                        m2 = Some(x);
                    } else if x < max2 {
                        if let Some(max3) = m3 {
                            if x > max3 {
                                m3 = Some(x);
                            }
                        } else {
                            m3 = Some(x);
                        }
                    }
                } else {
                    m2 = Some(x);
                }
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
fn name() {
    let nums = vec![3, 2, 1];
    assert_eq!(Solution::third_max(nums), 1);
    let nums = vec![1, 2];
    assert_eq!(Solution::third_max(nums), 2);
    let nums = vec![2, 2, 3, 1];
    assert_eq!(Solution::third_max(nums), 1);
    let nums = vec![1, 2, 2, 5, 3, 5];
    assert_eq!(Solution::third_max(nums), 2);
}
