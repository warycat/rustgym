struct Solution;

impl Solution {
    pub fn rotate_the_box(b: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let n = b.len();
        let m = b[0].len();
        let mut a = vec![];
        for _ in 0..m {
            a.push(vec!['.'; n]);
        }
        for i in 0..n {
            let mut x = m;
            let mut y = m;
            for _j in 0..m {
                match b[i][x - 1] {
                    '.' => {}
                    '*' => {
                        a[x - 1][n - 1 - i] = '*';
                        y = x - 1;
                    }
                    '#' => {
                        a[y - 1][n - 1 - i] = '#';
                        y -= 1;
                    }
                    _ => {}
                }
                x -= 1;
            }
        }
        a
    }
}

#[test]
fn test() {
    let b: Vec<Vec<char>> = vec_vec_char![['#', '.', '#']];
    let a: Vec<Vec<char>> = vec_vec_char![['.'], ['#'], ['#']];
    assert_eq!(Solution::rotate_the_box(b), a);
    let b: Vec<Vec<char>> = vec_vec_char![['#', '.', '*', '.'], ['#', '#', '*', '.']];
    let a: Vec<Vec<char>> = vec_vec_char![['#', '.'], ['#', '#'], ['*', '*'], ['.', '.']];
    assert_eq!(Solution::rotate_the_box(b), a)
}
