struct Solution;

impl Solution {
    fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }
        let nineteen: &'static str = "One Two Three Four Five Six Seven Eight Nine Ten Eleven Twelve Thirteen Fourteen Fifteen Sixteen Seventeen Eighteen Nineteen";
        let nineteen: Vec<&'static str> = nineteen.split_whitespace().collect();
        let tens: &'static str = "Twenty Thirty Forty Fifty Sixty Seventy Eighty Ninety";
        let tens: Vec<&'static str> = tens.split_whitespace().collect();
        let units: &'static str = "Hundred Thousand Million Billion";
        let units: Vec<&'static str> = units.split_whitespace().collect();
        Self::words(num as usize, &nineteen, &tens, &units).join(" ")
    }

    fn words(
        num: usize,
        nineteen: &[&'static str],
        tens: &[&'static str],
        units: &[&'static str],
    ) -> Vec<&'static str> {
        if num < 20 {
            if num > 0 {
                vec![nineteen[num - 1]]
            } else {
                vec![]
            }
        } else if num < 100 {
            let d = num / 10;
            vec![
                if d > 1 { vec![tens[d - 2]] } else { vec![] },
                Self::words(num % 10, nineteen, tens, units),
            ]
            .concat()
        } else if num < 1000 {
            vec![
                Self::words(num / 100, nineteen, tens, units),
                vec![units[0]],
                Self::words(num % 100, nineteen, tens, units),
            ]
            .concat()
        } else if num < 1_000_000 {
            vec![
                Self::words(num / 1000, nineteen, tens, units),
                vec![units[1]],
                Self::words(num % 1000, nineteen, tens, units),
            ]
            .concat()
        } else if num < 1_000_000_000 {
            vec![
                Self::words(num / 1_000_000, nineteen, tens, units),
                vec![units[2]],
                Self::words(num % 1_000_000, nineteen, tens, units),
            ]
            .concat()
        } else {
            vec![
                Self::words(num / 1_000_000_000, nineteen, tens, units),
                vec![units[3]],
                Self::words(num % 1_000_000_000, nineteen, tens, units),
            ]
            .concat()
        }
    }
}

#[test]
fn test() {
    let num = 123;
    let res = "One Hundred Twenty Three".to_string();
    assert_eq!(Solution::number_to_words(num), res);
    let num = 12345;
    let res = "Twelve Thousand Three Hundred Forty Five".to_string();
    assert_eq!(Solution::number_to_words(num), res);
    let num = 1_234_567;
    let res = "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven".to_string();
    assert_eq!(Solution::number_to_words(num), res);
    let num = 1_234_567_891;
    let res = "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One".to_string();
    assert_eq!(Solution::number_to_words(num), res);
    let num = 20;
    let res = "Twenty".to_string();
    assert_eq!(Solution::number_to_words(num), res);
}
