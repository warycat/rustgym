struct Solution;

impl Solution {
    fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();
        let mut i = 0;
        let mut j = 0;
        while i < g.len() && j < s.len() {
            if g[i] <= s[j] {
                i += 1;
                j += 1;
            } else {
                while j < s.len() && s[j] < g[i] {
                    j += 1;
                }
            }
        }
        i as i32
    }
}

#[test]
fn test() {
    let g = vec![1, 2, 3];
    let s = vec![1, 1];
    let res = 1;
    assert_eq!(Solution::find_content_children(g, s), res);
    let g = vec![1, 2];
    let s = vec![1, 2, 3];
    let res = 2;
    assert_eq!(Solution::find_content_children(g, s), res);
}
