struct Solution;

impl Solution {
    fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let i = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => panic!(),
        };
        items
            .into_iter()
            .filter(|item| item[i] == rule_value)
            .count() as i32
    }
}

#[test]
fn test() {
    let items = vec_vec_string![
        ["phone", "blue", "pixel"],
        ["computer", "silver", "lenovo"],
        ["phone", "gold", "iphone"]
    ];
    let rule_key = "color".to_string();
    let rule_value = "silver".to_string();
    let res = 1;
    assert_eq!(Solution::count_matches(items, rule_key, rule_value), res);
    let items = vec_vec_string![
        ["phone", "blue", "pixel"],
        ["computer", "silver", "phone"],
        ["phone", "gold", "iphone"]
    ];
    let rule_key = "type".to_string();
    let rule_value = "phone".to_string();
    let res = 2;
    assert_eq!(Solution::count_matches(items, rule_key, rule_value), res);
}
