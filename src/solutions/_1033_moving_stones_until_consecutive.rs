struct Solution;

impl Solution {
    fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut a: Vec<i32> = vec![a, b, c];
        a.sort_unstable();
        let min = if a[0] + 1 == a[1] && a[1] + 1 == a[2] {
            0
        } else {
            if a[0] + 2 >= a[1] || a[1] + 2 >= a[2] {
                1
            } else {
                2
            }
        };
        let max = a[1] - a[0] - 1 + a[2] - a[1] - 1;
        vec![min, max]
    }
}

#[test]
fn test() {
    let a = 1;
    let b = 2;
    let c = 5;
    let res = vec![1, 2];
    assert_eq!(Solution::num_moves_stones(a, b, c), res);
    let a = 4;
    let b = 3;
    let c = 2;
    let res = vec![0, 0];
    assert_eq!(Solution::num_moves_stones(a, b, c), res);
    let a = 3;
    let b = 5;
    let c = 1;
    let res = vec![1, 2];
    assert_eq!(Solution::num_moves_stones(a, b, c), res);
}
