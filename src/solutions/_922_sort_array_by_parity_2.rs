struct Solution;

impl Solution {
    fn sort_array_by_parity_ii(mut a: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut i: usize = 0;
        let mut j: usize = 1;
        while i < n && j < n {
            while i < n && a[i] % 2 == 0 {
                i += 2;
            }
            while j < n && a[j] % 2 == 1 {
                j += 2;
            }
            if i < n && j < n {
                a.swap(i, j);
            }
        }
        a
    }
}

#[test]
fn test() {
    let a = vec![4, 2, 5, 7];
    let b = vec![4, 5, 2, 7];

    assert_eq!(Solution::sort_array_by_parity_ii(a), b);
}
