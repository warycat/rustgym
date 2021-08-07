struct Solution;

impl Solution {
    fn minimize_error(prices: Vec<String>, target: i32) -> String {
        let prices: Vec<f64> = prices
            .into_iter()
            .map(|x| x.parse::<f64>().unwrap())
            .collect();
        let floor: i32 = prices.iter().map(|x| x.floor() as i32).sum();
        let ceil: i32 = prices.iter().map(|x| x.ceil() as i32).sum();
        let n = prices.len();
        if target < floor || target > ceil {
            return "-1".to_string();
        }
        let m = (target - floor) as usize;
        let mut diff = vec![];
        for i in 0..n {
            if prices[i].floor() as i32 != prices[i].ceil() as i32 {
                diff.push(prices[i].ceil() as f64 - prices[i]);
            }
        }
        let mut sum = 0.0;
        diff.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for i in 0..diff.len() {
            if i < m {
                sum += diff[i];
            } else {
                sum += 1.0 - diff[i];
            }
        }

        format!("{:.3}", sum)
    }
}

#[test]
fn test() {
    let prices = vec_string!["0.700", "2.800", "4.900"];
    let target = 8;
    let res = "1.000".to_string();
    assert_eq!(Solution::minimize_error(prices, target), res);
    let prices = vec_string!["1.500", "2.500", "3.500"];
    let target = 10;
    let res = "-1".to_string();
    assert_eq!(Solution::minimize_error(prices, target), res);
}
