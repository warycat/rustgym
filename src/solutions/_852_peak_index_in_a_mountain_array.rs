struct Solution;

impl Solution {
    fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = a.len() - 1;
        while l < r {
            let m = (l + r) / 2;
            if a[m] < a[m + 1] {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l as i32
    }
}

#[test]
fn test() {
    let a = vec![0, 1, 0];
    assert_eq!(Solution::peak_index_in_mountain_array(a), 1);
    let a = vec![0, 2, 1, 0];
    assert_eq!(Solution::peak_index_in_mountain_array(a), 1);
}
