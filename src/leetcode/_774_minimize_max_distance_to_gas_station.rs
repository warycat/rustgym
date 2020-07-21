struct Solution;

impl Solution {
    fn minmax_gas_dist(mut stations: Vec<i32>, k: i32) -> f64 {
        stations.sort_unstable();
        let stations: Vec<f64> = stations.into_iter().map(|x| x as f64).collect();
        let mut lo = 0.0;
        let mut hi = 1e8;
        while (hi - lo) > 1e-6 {
            let mi = (hi + lo) / 2.0;
            if Self::possible(mi, &stations, k) {
                hi = mi;
            } else {
                lo = mi;
            }
        }
        lo
    }

    fn possible(dist: f64, stations: &[f64], k: i32) -> bool {
        let mut count = 0;
        for i in 1..stations.len() {
            count += ((stations[i] - stations[i - 1]) / dist) as i32;
        }
        count <= k
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let stations = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let k = 9;
    let res = 0.5;
    assert_approx_eq!(Solution::minmax_gas_dist(stations, k), res);
}
