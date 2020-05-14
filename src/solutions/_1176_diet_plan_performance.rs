struct Solution;

impl Solution {
    fn update(sum: i32, lower: i32, upper: i32, point: &mut i32) {
        if sum < lower {
            *point -= 1;
        }
        if sum > upper {
            *point += 1;
        }
    }
    fn diet_plan_performance(calories: Vec<i32>, k: i32, lower: i32, upper: i32) -> i32 {
        let mut sum = 0;
        let mut point = 0;
        let n = calories.len();
        let k = k as usize;
        for i in 0..k {
            sum += calories[i];
        }
        Self::update(sum, lower, upper, &mut point);
        for i in 0..n - k {
            sum -= calories[i];
            sum += calories[i + k];
            Self::update(sum, lower, upper, &mut point);
        }
        point
    }
}

#[test]
fn test() {
    let calories: Vec<i32> = vec![1, 2, 3, 4, 5];
    let k = 1;
    let lower = 3;
    let upper = 3;
    let res = 0;
    assert_eq!(
        Solution::diet_plan_performance(calories, k, lower, upper),
        res
    );
    let calories: Vec<i32> = vec![3, 2];
    let k = 2;
    let lower = 0;
    let upper = 1;
    let res = 1;
    assert_eq!(
        Solution::diet_plan_performance(calories, k, lower, upper),
        res
    );
    let calories: Vec<i32> = vec![6, 5, 0, 0];
    let k = 2;
    let lower = 1;
    let upper = 5;
    let res = 0;
    assert_eq!(
        Solution::diet_plan_performance(calories, k, lower, upper),
        res
    );
}
