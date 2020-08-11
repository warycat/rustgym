struct Solution;

impl Solution {
    fn find_min_fibonacci_numbers(mut k: i32) -> i32 {
        let mut v = vec![1, 1];
        let mut i = 1;
        while v[i] + v[i - 1] <= k {
            v.push(v[i] + v[i - 1]);
            i += 1;
        }
        let mut count = 0;

        for x in v.into_iter().rev() {
            if x <= k {
                k -= x;
                count += 1;
            }
        }
        count
    }
}

#[test]
fn test() {
    let k = 7;
    let res = 2;
    assert_eq!(Solution::find_min_fibonacci_numbers(k), res);
    let k = 10;
    let res = 2;
    assert_eq!(Solution::find_min_fibonacci_numbers(k), res);
    let k = 19;
    let res = 3;
    assert_eq!(Solution::find_min_fibonacci_numbers(k), res);
}
