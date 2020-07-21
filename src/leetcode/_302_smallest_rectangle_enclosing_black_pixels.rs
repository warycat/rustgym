struct Solution;

impl Solution {
    fn min_area(image: Vec<Vec<char>>, _x: i32, _y: i32) -> i32 {
        let n = image.len();
        let m = image[0].len();
        let mut left = m;
        let mut right = 0;
        let mut top = n;
        let mut bottom = 0;
        for i in 0..n {
            for j in 0..m {
                if image[i][j] == '1' {
                    left = left.min(j);
                    right = right.max(j);
                    top = top.min(i);
                    bottom = bottom.max(i);
                }
            }
        }
        ((right - left + 1) * (bottom - top + 1)) as i32
    }
}

#[test]
fn test() {
    let image = vec_vec_char![
        ['0', '0', '1', '0'],
        ['0', '1', '1', '0'],
        ['0', '1', '0', '0']
    ];
    let x = 0;
    let y = 2;
    let res = 6;
    assert_eq!(Solution::min_area(image, x, y), res);
}
