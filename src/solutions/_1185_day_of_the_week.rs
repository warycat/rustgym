struct Solution;

impl Solution {
    fn day_of_the_week(day: i32, mut month: i32, mut year: i32) -> String {
        let days = [
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
        ];
        if month < 3 {
            month += 12;
            year -= 1;
        }
        let week =
            (day + ((month + 1) * 26) / 10 + year + year / 4 + 6 * (year / 100) + year / 400 + 5)
                % 7
                + 1;
        days[week as usize % 7].to_string()
    }
}

#[test]
fn test() {
    let day = 31;
    let month = 8;
    let year = 2019;
    let res = "Saturday".to_string();
    assert_eq!(Solution::day_of_the_week(day, month, year), res);
    let day = 18;
    let month = 7;
    let year = 1999;
    let res = "Sunday".to_string();
    assert_eq!(Solution::day_of_the_week(day, month, year), res);
    let day = 15;
    let month = 8;
    let year = 1993;
    let res = "Sunday".to_string();
    assert_eq!(Solution::day_of_the_week(day, month, year), res);
}
