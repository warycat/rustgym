struct Solution;

impl Solution {
    fn days_between_dates(date1: String, date2: String) -> i32 {
        let (y1, m1, d1) = Self::parse(date1);
        let (y2, m2, d2) = Self::parse(date2);
        let mut s1 = 0;
        let mut s2 = 0;
        for i in 1971..y1 {
            s1 += if Self::is_leap(i) { 366 } else { 365 };
        }
        for i in 1971..y2 {
            s2 += if Self::is_leap(i) { 366 } else { 365 };
        }
        s1 += Self::day_of_year(y1, m1, d1);
        s2 += Self::day_of_year(y2, m2, d2);
        (s1 as i32 - s2 as i32).abs()
    }

    fn parse(date: String) -> (usize, usize, usize) {
        let a: Vec<&str> = date.split_terminator('-').collect();
        let year = a[0].parse::<usize>().unwrap();
        let month = a[1].parse::<usize>().unwrap();
        let day = a[2].parse::<usize>().unwrap();
        (year, month, day)
    }

    fn is_leap(year: usize) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }

    fn day_of_year(year: usize, month: usize, day: usize) -> usize {
        let mut days: Vec<usize> = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut sum = 0;
        if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
            days[1] += 1;
        }
        for i in 0..month - 1 {
            sum += days[i];
        }
        sum += day;
        sum
    }
}

#[test]
fn test() {
    let date1 = "2019-06-29".to_string();
    let date2 = "2019-06-30".to_string();
    let res = 1;
    assert_eq!(Solution::days_between_dates(date1, date2), res);
    let date1 = "2020-01-15".to_string();
    let date2 = "2019-12-31".to_string();
    let res = 15;
    assert_eq!(Solution::days_between_dates(date1, date2), res);
}
