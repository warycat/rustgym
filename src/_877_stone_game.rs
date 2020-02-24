struct Solution;

impl Solution {
    fn stone_game(_: Vec<i32>) -> bool {
        true
    }
}

#[test]
fn test() {
    let piles = vec![5, 3, 4, 5];
    let res = true;
    assert_eq!(Solution::stone_game(piles), res);
}
