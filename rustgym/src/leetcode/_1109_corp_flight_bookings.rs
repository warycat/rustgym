struct Solution;

impl Solution {
    fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![0; n];
        for booking in bookings {
            let start = (booking[0] - 1) as usize;
            let end = booking[1] as usize;
            res[start] += booking[2];
            if end < n {
                res[end] -= booking[2];
            }
        }
        let mut prev = 0;
        for i in 0..n {
            prev += res[i];
            res[i] = prev;
        }
        res
    }
}

#[test]
fn test() {
    let bookings = vec_vec_i32![[1, 2, 10], [2, 3, 20], [2, 5, 25]];
    let n = 5;
    let res = vec![10, 55, 45, 25, 25];
    assert_eq!(Solution::corp_flight_bookings(bookings, n), res);
}
