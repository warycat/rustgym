struct Solution;

impl Solution {
    fn get_strongest(mut arr: Vec<i32>, mut k: i32) -> Vec<i32> {
        arr.sort_unstable();
        let n = arr.len();
        let median = arr[(n - 1) / 2];
        let mut l = 0;
        let mut r = n - 1;
        let mut res = vec![];
        while k > 0 {
            if (arr[l] - median).abs() <= (arr[r] - median).abs() {
                res.push(arr[r]);
                r -= 1;
            } else {
                res.push(arr[l]);
                l += 1;
            }
            k -= 1;
        }
        res
    }
}

#[test]
fn test() {
    let arr = vec![1, 2, 3, 4, 5];
    let k = 2;
    let mut res = vec![5, 1];
    let mut ans = Solution::get_strongest(arr, k);
    res.sort_unstable();
    ans.sort_unstable();
    assert_eq!(ans, res);
    let arr = vec![1, 1, 3, 5, 5];
    let k = 2;
    let mut res = vec![5, 5];
    let mut ans = Solution::get_strongest(arr, k);
    res.sort_unstable();
    ans.sort_unstable();
    assert_eq!(ans, res);
    let arr = vec![6, 7, 11, 7, 6, 8];
    let k = 5;
    let mut res = vec![11, 8, 6, 6, 7];
    let mut ans = Solution::get_strongest(arr, k);
    res.sort_unstable();
    ans.sort_unstable();
    assert_eq!(ans, res);
    let arr = vec![6, -3, 7, 2, 11];
    let k = 3;
    let mut res = vec![-3, 11, 2];
    let mut ans = Solution::get_strongest(arr, k);
    res.sort_unstable();
    ans.sort_unstable();
    assert_eq!(ans, res);
    let arr = vec![-7, 22, 17, 3];
    let k = 2;
    let mut res = vec![22, 17];
    let mut ans = Solution::get_strongest(arr, k);
    res.sort_unstable();
    ans.sort_unstable();
    assert_eq!(ans, res);
}
