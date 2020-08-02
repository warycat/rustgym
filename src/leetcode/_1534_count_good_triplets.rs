struct Solution;

impl Solution {
    fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let n = arr.len();
        let mut res = 0;
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    if (arr[i] - arr[j]).abs() <= a
                        && (arr[j] - arr[k]).abs() <= b
                        && (arr[i] - arr[k]).abs() <= c
                    {
                        res += 1;
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let arr = vec![3, 0, 1, 1, 9, 7];
    let a = 7;
    let b = 2;
    let c = 3;
    let res = 4;
    assert_eq!(Solution::count_good_triplets(arr, a, b, c), res);
    let arr = vec![1, 1, 2, 2, 3];
    let a = 0;
    let b = 0;
    let c = 1;
    let res = 0;
    assert_eq!(Solution::count_good_triplets(arr, a, b, c), res);
}
