struct Solution;

impl Solution {
    fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_unstable_by_key(|&x| (x.count_ones(), x));
        arr
    }
}

#[test]
fn test() {
    let arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let res = vec![0, 1, 2, 4, 8, 3, 5, 6, 7];
    assert_eq!(Solution::sort_by_bits(arr), res);
    let arr = vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1];
    let res = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
    assert_eq!(Solution::sort_by_bits(arr), res);
    let arr = vec![10000, 10000];
    let res = vec![10000, 10000];
    assert_eq!(Solution::sort_by_bits(arr), res);
    let arr = vec![2, 3, 5, 7, 11, 13, 17, 19];
    let res = vec![2, 3, 5, 17, 7, 11, 13, 19];
    assert_eq!(Solution::sort_by_bits(arr), res);
    let arr = vec![10, 100, 1000, 10000];
    let res = vec![10, 100, 10000, 1000];
    assert_eq!(Solution::sort_by_bits(arr), res);
}
