struct Solution;

impl Solution {
    fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        rec1[2] > rec2[0] && rec1[0] < rec2[2] && rec1[1] < rec2[3] && rec1[3] > rec2[1]
    }
}

#[test]
fn test() {
    let rec1 = vec![0, 0, 2, 2];
    let rec2 = vec![1, 1, 3, 3];
    assert_eq!(Solution::is_rectangle_overlap(rec1, rec2), true);
    let rec1 = vec![0, 0, 1, 1];
    let rec2 = vec![1, 0, 2, 1];
    assert_eq!(Solution::is_rectangle_overlap(rec1, rec2), false);
}
