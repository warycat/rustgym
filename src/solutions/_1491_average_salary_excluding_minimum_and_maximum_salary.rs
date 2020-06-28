struct Solution;

impl Solution {
    fn average(salary: Vec<i32>) -> f64 {
        let n = salary.len();
        let min = *salary.iter().min().unwrap();
        let max = *salary.iter().max().unwrap();
        let sum: i32 = salary.into_iter().filter(|&x| x > min && x < max).sum();
        sum as f64 / (n - 2) as f64
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let salary = vec![4000, 3000, 1000, 2000];
    let res = 2500.00000;
    assert_approx_eq!(Solution::average(salary), res);
    let salary = vec![1000, 2000, 3000];
    let res = 2000.00000;
    assert_approx_eq!(Solution::average(salary), res);
    let salary = vec![6000, 5000, 4000, 3000, 2000, 1000];
    let res = 3500.00000;
    assert_approx_eq!(Solution::average(salary), res);
    let salary = vec![8000, 9000, 2000, 3000, 6000, 1000];
    let res = 4750.00000;
    assert_approx_eq!(Solution::average(salary), res);
}
