struct Solution;

impl Solution {
    fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut start = std::i32::MAX;
        let mut end = 0;
        for log in &logs {
            let birth = log[0];
            let death = log[1];
            start = start.min(birth);
            end = end.max(death);
        }
        let mut res = 0;
        let mut max = 0;
        for i in start..end {
            let mut population = 0;
            for log in &logs {
                let birth = log[0];
                let death = log[1];
                if (birth..death).contains(&i) {
                    population += 1;
                }
            }
            if max < population {
                res = i;
                max = population;
            }
        }
        res
    }
}

#[test]
fn test() {
    let logs = vec_vec_i32![[1993, 1999], [2000, 2010]];
    let res = 1993;
    assert_eq!(Solution::maximum_population(logs), res);
    let logs = vec_vec_i32![[1950, 1961], [1960, 1971], [1970, 1981]];
    let res = 1960;
    assert_eq!(Solution::maximum_population(logs), res);
}
