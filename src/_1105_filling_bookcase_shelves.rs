struct Solution;

impl Solution {
    fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let n = books.len();
        let mut dp = vec![0; n + 1];
        for i in 0..n {
            let mut width = books[i][0];
            let mut height = books[i][1];
            dp[i + 1] = height + dp[i];
            for j in (0..i).rev() {
                width += books[j][0];
                if width <= shelf_width {
                    height = height.max(books[j][1]);
                    dp[i + 1] = dp[i + 1].min(height + dp[j]);
                }
            }
        }
        dp[n]
    }
}

#[test]
fn test() {
    let books = vec_vec_i32![[1, 1], [2, 3], [2, 3], [1, 1], [1, 1], [1, 1], [1, 2]];
    let shelf_width = 4;
    let res = 6;
    assert_eq!(Solution::min_height_shelves(books, shelf_width), res);
}
