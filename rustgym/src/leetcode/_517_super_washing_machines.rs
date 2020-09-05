struct Solution;

impl Solution {
    fn find_min_moves(machines: Vec<i32>) -> i32 {
        let n = machines.len();
        let sum: i32 = machines.iter().sum();
        if sum % n as i32 != 0 {
            return -1;
        }
        let avg = sum / n as i32;
        let mut res = 0;
        let mut count = 0;
        for i in (1..n).rev() {
            let diff = machines[i] - avg;
            res = res.max(diff);
            count += diff;
            res = res.max(count.abs());
        }
        res
    }
}

#[test]
fn test() {
    let machines = vec![1, 0, 5];
    let res = 3;
    assert_eq!(Solution::find_min_moves(machines), res);
    let machines = vec![0, 3, 0];
    let res = 2;
    assert_eq!(Solution::find_min_moves(machines), res);
    let machines = vec![0, 2, 0];
    let res = -1;
    assert_eq!(Solution::find_min_moves(machines), res);
}
