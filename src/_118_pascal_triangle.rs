struct Solution;

impl Solution {
    fn generate(nums_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        for i in 0..nums_rows {
            let ui = i as usize;
            res.push(vec![]);
            for j in 0..=i {
                let uj = j as usize;
                if j == 0 || j == i {
                    res[ui].push(1);
                } else {
                    let prev = &res[ui - 1];
                    let sum = prev[uj - 1] + prev[uj];
                    res[ui].push(sum);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let triangle_5 = vec![
        vec![1],
        vec![1, 1],
        vec![1, 2, 1],
        vec![1, 3, 3, 1],
        vec![1, 4, 6, 4, 1],
    ];
    assert_eq!(Solution::generate(5), triangle_5);
}
