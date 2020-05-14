struct Solution;

impl Solution {
    fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        let mut l = 0;
        let mut r = a.len() - 1;
        while l < r {
            while a[l] % 2 == 0 && l < r {
                l += 1;
            }

            while a[r] % 2 == 1 && l < r {
                r -= 1;
            }

            if l < r {
                a.swap(l, r);
            }
        }
        a
    }
}

#[test]
fn test() {
    let a = vec![3, 1, 2, 4];
    let b = vec![4, 2, 1, 3];
    assert_eq!(Solution::sort_array_by_parity(a), b);
}
