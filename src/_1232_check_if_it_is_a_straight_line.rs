struct Solution;

impl Solution {
    fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let n = coordinates.len();
        if n == 2 {
            return true;
        }
        let x1 = coordinates[1][0] - coordinates[0][0];
        let y1 = coordinates[1][1] - coordinates[0][1];
        for i in 1..n {
            let xi = coordinates[i][0] - coordinates[0][0];
            let yi = coordinates[i][1] - coordinates[0][1];
            if x1 * yi != xi * y1 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let coordinates: Vec<Vec<i32>> = vec_vec_i32![[1, 2], [2, 3], [3, 4], [4, 5], [5, 6], [6, 7]];
    assert_eq!(Solution::check_straight_line(coordinates), true);
    let coordinates: Vec<Vec<i32>> = vec_vec_i32![[1, 1], [2, 2], [3, 4], [4, 5], [5, 6], [7, 7]];
    assert_eq!(Solution::check_straight_line(coordinates), false);
}
