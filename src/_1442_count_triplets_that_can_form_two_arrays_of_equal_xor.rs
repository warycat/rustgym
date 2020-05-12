struct Solution;

impl Solution {
    fn count_triplets(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut res = 0;
        for i in 0..n {
            let mut xor = arr[i];
            for j in i + 1..n {
                xor ^= arr[j];
                if xor == 0 {
                    res += j - i;
                }
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let arr = vec![2, 3, 1, 6, 7];
    let res = 4;
    assert_eq!(Solution::count_triplets(arr), res);
    let arr = vec![1, 1, 1, 1, 1];
    let res = 10;
    assert_eq!(Solution::count_triplets(arr), res);
    let arr = vec![2, 3];
    let res = 0;
    assert_eq!(Solution::count_triplets(arr), res);
    let arr = vec![1, 3, 5, 7, 9];
    let res = 3;
    assert_eq!(Solution::count_triplets(arr), res);
    let arr = vec![7, 11, 12, 9, 5, 2, 7, 17, 22];
    let res = 8;
    assert_eq!(Solution::count_triplets(arr), res);
}
