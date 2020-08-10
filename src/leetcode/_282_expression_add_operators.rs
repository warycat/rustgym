struct Solution;

impl Solution {
    fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut res = vec![];
        for i in 1..=num.len() {
            let expr = &num[0..i];
            let val = expr.parse::<i64>().unwrap();
            if expr.len() > 1 && &expr[0..1] == "0" {
                break;
            }
            Self::dfs(
                i,
                expr.to_string(),
                val,
                val,
                '#',
                &mut res,
                &num,
                target as i64,
            );
        }
        res
    }

    fn dfs(
        start: usize,
        expr: String,
        val: i64,
        prev_val: i64,
        prev_op: char,
        all: &mut Vec<String>,
        num: &str,
        target: i64,
    ) {
        if start == num.len() {
            if val == target {
                all.push(expr);
            }
        } else {
            for end in start + 1..=num.len() {
                let cur_expr = &num[start..end];
                let cur_val = cur_expr.parse::<i64>().unwrap();
                if cur_expr.len() > 1 && &cur_expr[0..1] == "0" {
                    break;
                }
                Self::dfs(
                    end,
                    format!("{}+{}", expr, cur_expr),
                    val + cur_val,
                    cur_val,
                    '+',
                    all,
                    num,
                    target,
                );
                Self::dfs(
                    end,
                    format!("{}-{}", expr, cur_expr),
                    val - cur_val,
                    cur_val,
                    '-',
                    all,
                    num,
                    target,
                );
                Self::dfs(
                    end,
                    format!("{}*{}", expr, cur_expr),
                    match prev_op {
                        '+' => val - prev_val + prev_val * cur_val,
                        '-' => val + prev_val - prev_val * cur_val,
                        _ => prev_val * cur_val,
                    },
                    prev_val * cur_val,
                    prev_op,
                    all,
                    num,
                    target,
                )
            }
        }
    }
}

#[test]
fn test() {
    let num = "123".to_string();
    let target = 6;
    let mut res = vec_string!["1+2+3", "1*2*3"];
    let mut ans = Solution::add_operators(num, target);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let num = "232".to_string();
    let target = 8;
    let mut res = vec_string!["2*3+2", "2+3*2"];
    let mut ans = Solution::add_operators(num, target);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let num = "105".to_string();
    let target = 5;
    let mut res = vec_string!["1*0+5", "10-5"];
    let mut ans = Solution::add_operators(num, target);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let num = "00".to_string();
    let target = 0;
    let mut res = vec_string!["0+0", "0-0", "0*0"];
    let mut ans = Solution::add_operators(num, target);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let num = "3456237490".to_string();
    let target = 9191;
    let mut res = vec_string![];
    let mut ans = Solution::add_operators(num, target);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
