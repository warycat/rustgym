struct Solution;

impl Solution {
    fn day_of_year(date: String) -> i32 {
        let a: Vec<&str> = date.split_terminator('-').collect();
        let year = a[0].parse::<usize>().unwrap();
        let month = a[1].parse::<usize>().unwrap();
        let day = a[2].parse::<usize>().unwrap();
        let mut days: Vec<usize> = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut sum = 0;
        if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
            days[1] += 1;
        }
        for i in 0..month - 1 {
            sum += days[i];
        }
        sum += day;
        sum as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::day_of_year("2019-01-09".to_string()), 9);
    assert_eq!(Solution::day_of_year("2019-02-10".to_string()), 41);
    assert_eq!(Solution::day_of_year("2003-03-01".to_string()), 60);
    assert_eq!(Solution::day_of_year("2004-03-01".to_string()), 61);
}
