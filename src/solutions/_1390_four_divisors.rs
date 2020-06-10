struct Solution;

impl Solution {
    fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        'outer: for x in nums {
            let mut i = 2;
            let mut v = vec![];
            while i * i <= x {
                if x % i == 0 {
                    v.push(i);
                    if v.len() > 1 {
                        continue 'outer;
                    }
                }
                i += 1;
            }
            if v.len() == 1 && v[0] * v[0] != x {
                res += 1 + v[0] + x / v[0] + x;
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![21, 4, 7];
    let res = 32;
    assert_eq!(Solution::sum_four_divisors(nums), res);
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let res = 45;
    assert_eq!(Solution::sum_four_divisors(nums), res);
}
