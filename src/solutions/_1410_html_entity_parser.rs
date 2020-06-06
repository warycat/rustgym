struct Solution;

impl Solution {
    fn entity_parser(text: String) -> String {
        text.replace("&quot;", "\"")
            .replace("&apos;", "'")
            .replace("&frasl;", "/")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&amp;", "&")
    }
}

#[test]
fn test() {
    let text = "&amp; is an HTML entity but &ambassador; is not.".to_string();
    let res = "& is an HTML entity but &ambassador; is not.".to_string();
    assert_eq!(Solution::entity_parser(text), res);
    let text = "and I quote: &quot;...&quot;".to_string();
    let res = "and I quote: \"...\"".to_string();
    assert_eq!(Solution::entity_parser(text), res);
    let text = "Stay home! Practice on Leetcode :)".to_string();
    let res = "Stay home! Practice on Leetcode :)".to_string();
    assert_eq!(Solution::entity_parser(text), res);
    let text = "x &gt; y &amp;&amp; x &lt; y is always false".to_string();
    let res = "x > y && x < y is always false".to_string();
    assert_eq!(Solution::entity_parser(text), res);
    let text = "leetcode.com&frasl;problemset&frasl;all".to_string();
    let res = "leetcode.com/problemset/all".to_string();
    assert_eq!(Solution::entity_parser(text), res);
}
