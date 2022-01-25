struct Solution;

impl Solution {
    fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort_unstable();
        students.sort_unstable();
        let n = seats.len();
        let mut res = 0;
        for i in 0..n {
            res += (seats[i] - students[i]).abs();
        }
        res
    }
}

#[test]
fn test() {
    let seats = vec![3, 1, 5];
    let students = vec![2, 7, 4];
    let res = 4;
    assert_eq!(Solution::min_moves_to_seat(seats, students), res);
    let seats = vec![4, 1, 5, 9];
    let students = vec![1, 3, 2, 6];
    let res = 7;
    assert_eq!(Solution::min_moves_to_seat(seats, students), res);
    let seats = vec![2, 2, 6, 6];
    let students = vec![1, 3, 2, 6];
    let res = 4;
    assert_eq!(Solution::min_moves_to_seat(seats, students), res);
}
