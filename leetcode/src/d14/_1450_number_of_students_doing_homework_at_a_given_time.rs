struct Solution;

impl Solution {
    fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let n = start_time.len();
        let mut res = 0;
        for i in 0..n {
            let start = start_time[i];
            let end = end_time[i];
            if start <= query_time && end >= query_time {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let start_time = vec![1, 2, 3];
    let end_time = vec![3, 2, 7];
    let query_time = 4;
    let res = 1;
    assert_eq!(
        Solution::busy_student(start_time, end_time, query_time),
        res
    );
    let start_time = vec![4];
    let end_time = vec![4];
    let query_time = 4;
    let res = 1;
    assert_eq!(
        Solution::busy_student(start_time, end_time, query_time),
        res
    );
    let start_time = vec![4];
    let end_time = vec![4];
    let query_time = 5;
    let res = 0;
    assert_eq!(
        Solution::busy_student(start_time, end_time, query_time),
        res
    );
    let start_time = vec![1, 1, 1, 1];
    let end_time = vec![1, 3, 2, 4];
    let query_time = 7;
    let res = 0;
    assert_eq!(
        Solution::busy_student(start_time, end_time, query_time),
        res
    );
    let start_time = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    let end_time = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
    let query_time = 5;
    let res = 5;
    assert_eq!(
        Solution::busy_student(start_time, end_time, query_time),
        res
    );
}
