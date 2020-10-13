struct Solution;

impl Solution {
    fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let n = costs.len();
        if n == 0 {
            return 0;
        }
        let mut r = costs[0][0];
        let mut g = costs[0][1];
        let mut b = costs[0][2];
        for i in 1..n {
            let rr = i32::min(g, b) + costs[i][0];
            let gg = i32::min(r, b) + costs[i][1];
            let bb = i32::min(r, g) + costs[i][2];
            r = rr;
            g = gg;
            b = bb;
        }
        let mut res: Vec<i32> = [r, g, b].to_vec();
        res.sort_unstable();
        res[0]
    }
}

#[test]
fn test() {
    let costs: Vec<Vec<i32>> = vec_vec_i32![[17, 2, 17], [16, 16, 5], [14, 3, 19]];
    assert_eq!(Solution::min_cost(costs), 10);
}
