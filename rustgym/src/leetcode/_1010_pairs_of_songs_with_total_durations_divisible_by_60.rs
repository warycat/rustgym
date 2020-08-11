struct Solution;

impl Solution {
    fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut a: Vec<i32> = vec![0; 60];
        let mut res = 0;
        for x in time {
            let count = a[((600 - x) % 60) as usize];
            if count != 0 {
                res += count;
            }
            a[(x % 60) as usize] += 1;
        }
        res
    }
}

#[test]
fn test() {
    let time = vec![30, 20, 150, 100, 40];
    assert_eq!(Solution::num_pairs_divisible_by60(time), 3);
    let time = vec![60, 60, 60];
    assert_eq!(Solution::num_pairs_divisible_by60(time), 3);
}
