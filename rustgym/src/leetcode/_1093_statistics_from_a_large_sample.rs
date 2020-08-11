struct Solution;

impl Solution {
    fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        let mut min = std::i32::MAX;
        let mut max = std::i32::MIN;
        let total_count: i32 = count.iter().copied().sum();
        let m1 = (total_count + 1) / 2;
        let m2 = if total_count % 2 == 0 { m1 + 1 } else { m1 };
        let mut cur_count = 0;
        let mut mode_count = 0;
        let mut mode = std::i32::MAX;
        let mut sum = 0.0;
        let n = count.len();
        let mut median = 0;
        for i in 0..n {
            if count[i] > 0 {
                min = min.min(i as i32);
                max = max.max(i as i32);
                if cur_count < m1 && cur_count + count[i] >= m1 {
                    median += i;
                }
                if cur_count < m2 && cur_count + count[i] >= m2 {
                    median += i;
                }
                cur_count += count[i];
                sum += i as f64 * count[i] as f64;
                if count[i] > mode_count {
                    mode_count = count[i];
                    mode = i as i32;
                }
            }
        }
        vec![
            min as f64,
            max as f64,
            sum / total_count as f64,
            median as f64 / 2.0,
            mode as f64,
        ]
    }
}

#[test]
fn test() {
    let count = vec![
        0, 1, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let res = vec![1.00000, 3.00000, 2.37500, 2.50000, 3.00000];
    assert_eq!(Solution::sample_stats(count), res);
}
