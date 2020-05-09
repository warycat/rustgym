struct Solution;

impl Solution {
    fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        queries
            .into_iter()
            .map(|query| Self::query_match(query.chars().collect(), pattern.chars().collect()))
            .collect()
    }

    fn query_match(query: Vec<char>, pattern: Vec<char>) -> bool {
        let mut j = 0;
        for i in 0..query.len() {
            if j < pattern.len() && query[i] == pattern[j] {
                j += 1;
            } else {
                if query[i].is_uppercase() {
                    return false;
                }
            }
        }
        j == pattern.len()
    }
}

#[test]
fn test() {
    let queries = vec_string![
        "FooBar",
        "FooBarTest",
        "FootBall",
        "FrameBuffer",
        "ForceFeedBack"
    ];
    let pattern = "FB".to_string();
    let res = vec![true, false, true, true, false];
    assert_eq!(Solution::camel_match(queries, pattern), res);
    let queries = vec_string![
        "FooBar",
        "FooBarTest",
        "FootBall",
        "FrameBuffer",
        "ForceFeedBack"
    ];
    let pattern = "FoBa".to_string();
    let res = vec![true, false, true, false, false];
    assert_eq!(Solution::camel_match(queries, pattern), res);
    let queries = vec_string![
        "FooBar",
        "FooBarTest",
        "FootBall",
        "FrameBuffer",
        "ForceFeedBack"
    ];
    let pattern = "FoBaT".to_string();
    let res = vec![false, true, false, false, false];
    assert_eq!(Solution::camel_match(queries, pattern), res);
}
