struct Solution;

impl Solution {
    fn count_bits(num: i32) -> Vec<i32> {
        let n = num as usize;
        let mut res = vec![];
        for i in 0..=n {
            res.push(i.count_ones() as i32);
        }
        res
    }
}

#[test]
fn test() {
    let num = 2;
    let res = vec![0, 1, 1];
    assert_eq!(Solution::count_bits(num), res);
}
