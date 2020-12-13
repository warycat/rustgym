struct Solution;

impl Solution {
    fn max_height(mut cuboids: Vec<Vec<i32>>) -> i32 {
        let n = cuboids.len();
        for i in 0..n {
            cuboids[i].sort_unstable();
        }
        cuboids.sort_unstable();
        let mut dp: Vec<i32> = vec![0; n];
        for i in 0..n {
            dp[i] = cuboids[i][2];
            'outer: for j in 0..i {
                for k in 0..3 {
                    if cuboids[j][k] > cuboids[i][k] {
                        continue 'outer;
                    }
                }
                dp[i] = dp[i].max(dp[j] + cuboids[i][2]);
            }
        }
        dp.into_iter().max().unwrap()
    }
}

#[test]
fn test() {
    let cuboids = vec_vec_i32![[50, 45, 20], [95, 37, 53], [45, 23, 12]];
    let res = 190;
    assert_eq!(Solution::max_height(cuboids), res);
    let cuboids = vec_vec_i32![[38, 25, 45], [76, 35, 3]];
    let res = 76;
    assert_eq!(Solution::max_height(cuboids), res);
    let cuboids = vec_vec_i32![
        [7, 11, 17],
        [7, 17, 11],
        [11, 7, 17],
        [11, 17, 7],
        [17, 7, 11],
        [17, 11, 7]
    ];
    let res = 102;
    assert_eq!(Solution::max_height(cuboids), res);
}
