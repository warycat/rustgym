struct Solution;

impl Solution {
    fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut chef: f64 = 0.0;
        let mut total_wait_time: f64 = 0.0;
        let n = customers.len();
        for customer in customers {
            let arrival = customer[0] as f64;
            let time = customer[1] as f64;
            let start_time = chef.max(arrival);
            let end_time = start_time + time;
            chef = end_time;
            let wait_time = end_time - arrival;
            total_wait_time += wait_time;
        }
        total_wait_time / n as f64
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let customers = vec_vec_i32![[1, 2], [2, 5], [4, 3]];
    let res = 5.0;
    assert_approx_eq!(Solution::average_waiting_time(customers), res);
    let customers = vec_vec_i32![[5, 2], [5, 4], [10, 3], [20, 1]];
    let res = 3.25000;
    assert_approx_eq!(Solution::average_waiting_time(customers), res);
}
