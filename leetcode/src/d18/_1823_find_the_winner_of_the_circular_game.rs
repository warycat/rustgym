struct Solution;

impl Solution {
    fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut friends = vec![];
        for i in 1..=n {
            friends.push(i);
        }
        let mut i = 0;
        loop {
            let m = friends.len();
            if m == 1 {
                break;
            }
            i = (i as i32 + k - 1) as usize % m;
            friends.remove(i);
            if i == m - 1 {
                i = 0;
            }
        }
        friends[0]
    }
}

#[test]
fn test() {
    let n = 5;
    let k = 2;
    let res = 3;
    assert_eq!(Solution::find_the_winner(n, k), res);
    let n = 6;
    let k = 5;
    let res = 1;
    assert_eq!(Solution::find_the_winner(n, k), res);
}
