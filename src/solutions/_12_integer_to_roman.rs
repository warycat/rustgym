struct Solution;

impl Solution {
    fn int_to_roman(mut num: i32) -> String {
        let mapping: Vec<(i32, &str)> = vec![
            (1, "I"),
            (2, "II"),
            (3, "III"),
            (4, "IV"),
            (5, "V"),
            (6, "VI"),
            (7, "VII"),
            (8, "VIII"),
            (9, "IX"),
            (10, "X"),
            (20, "XX"),
            (30, "XXX"),
            (40, "XL"),
            (50, "L"),
            (60, "LX"),
            (70, "LXX"),
            (80, "LXXX"),
            (90, "XC"),
            (100, "C"),
            (200, "CC"),
            (300, "CCC"),
            (400, "CD"),
            (500, "D"),
            (600, "DC"),
            (700, "DCC"),
            (800, "DCCC"),
            (900, "CM"),
            (1000, "M"),
            (2000, "MM"),
            (3000, "MMM"),
        ];
        let mut res: String = "".to_string();
        for (x, s) in mapping.iter().rev() {
            if num >= *x {
                res += s;
                num -= x;
            } else {
                continue;
            }
            if num == 0 {
                break;
            }
        }
        res
    }
}

#[test]
fn test() {
    let num = 3;
    let res = "III".to_string();
    assert_eq!(Solution::int_to_roman(num), res);
    let num = 1994;
    let res = "MCMXCIV".to_string();
    assert_eq!(Solution::int_to_roman(num), res);
}
