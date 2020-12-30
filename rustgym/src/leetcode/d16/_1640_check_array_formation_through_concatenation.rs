struct Solution;

impl Solution {
    fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let n = arr.len();
        Self::dp(n, &arr, &pieces)
    }

    fn dp(n: usize, arr: &[i32], pieces: &[Vec<i32>]) -> bool {
        if n == 0 {
            return true;
        }
        for piece in pieces {
            let m = piece.len();
            if m <= n && arr[n - m..n] == piece[0..m] && Self::dp(n - m, arr, pieces) {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    let arr = vec![85];
    let pieces = vec_vec_i32![[85]];
    let res = true;
    assert_eq!(Solution::can_form_array(arr, pieces), res);
    let arr = vec![15, 88];
    let pieces = vec_vec_i32![[88], [15]];
    let res = true;
    assert_eq!(Solution::can_form_array(arr, pieces), res);
    let arr = vec![49, 18, 16];
    let pieces = vec_vec_i32![[16, 18, 49]];
    let res = false;
    assert_eq!(Solution::can_form_array(arr, pieces), res);
    let arr = vec![91, 4, 64, 78];
    let pieces = vec_vec_i32![[78], [4, 64], [91]];
    let res = true;
    assert_eq!(Solution::can_form_array(arr, pieces), res);
    let arr = vec![1, 3, 5, 7];
    let pieces = vec_vec_i32![[2, 4, 6, 8]];
    let res = false;
    assert_eq!(Solution::can_form_array(arr, pieces), res);
}
