struct Solution;

impl Solution {
    fn max_area(h: i32, w: i32, mut horizontal_cuts: Vec<i32>, mut vertical_cuts: Vec<i32>) -> i32 {
        horizontal_cuts.push(0);
        horizontal_cuts.push(h);
        horizontal_cuts.sort_unstable();
        vertical_cuts.push(0);
        vertical_cuts.push(w);
        vertical_cuts.sort_unstable();
        let h = horizontal_cuts
            .windows(2)
            .map(|v| v[1] - v[0])
            .max()
            .unwrap();
        let w = vertical_cuts.windows(2).map(|v| v[1] - v[0]).max().unwrap();
        ((h as i64 * w as i64) % 1_000_000_007) as i32
    }
}

#[test]
fn test() {
    let h = 5;
    let w = 4;
    let horizontal_cuts = vec![1, 2, 4];
    let vertical_cuts = vec![1, 3];
    let res = 4;
    assert_eq!(
        Solution::max_area(h, w, horizontal_cuts, vertical_cuts),
        res
    );
}
