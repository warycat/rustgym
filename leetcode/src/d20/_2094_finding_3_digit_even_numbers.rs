struct Solution;

impl Solution {
    fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut all = [0; 10];
        for d in digits {
            all[d as usize] += 1;
        }
        let mut res = vec![];
        let even = vec![0, 2, 4, 6, 8];
        for i in 1..=9 {
            for j in 0..=9 {
                for &k in &even {
                    let mut count = [0; 10];
                    count[i] += 1;
                    count[j] += 1;
                    count[k] += 1;
                    if count[i] <= all[i] && count[j] <= all[j] && count[k] <= all[k] {
                        res.push((i * 100 + j * 10 + k) as i32);
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let digits = vec![2, 1, 3, 0];
    let res = vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320];
    assert_eq!(Solution::find_even_numbers(digits), res);
    let digits = vec![2, 2, 8, 8, 2];
    let res = vec![222, 228, 282, 288, 822, 828, 882];
    assert_eq!(Solution::find_even_numbers(digits), res);
    let digits = vec![3, 7, 5];
    let res: Vec<i32> = vec![];
    assert_eq!(Solution::find_even_numbers(digits), res);
}
