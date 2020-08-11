struct Solution;

use std::collections::VecDeque;

struct Pixel {
    i: usize,
    j: usize,
}

impl Solution {
    fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let n = image.len();
        if n == 0 {
            return image;
        }
        let m = image[0].len();
        if m == 0 {
            return image;
        }
        if sr < 0 {
            return image;
        }
        let sr = sr as usize;
        if sr >= n {
            return image;
        }
        if sc < 0 {
            return image;
        }
        let sc = sc as usize;
        if sc >= m {
            return image;
        }
        let c = image[sr][sc];
        if c == new_color {
            return image;
        }
        let mut queue: VecDeque<Pixel> = VecDeque::new();
        queue.push_back(Pixel { i: sr, j: sc });
        let di = vec![0, 0, -1, 1];
        let dj = vec![-1, 1, 0, 0];
        while let Some(pixel) = queue.pop_front() {
            let i = pixel.i;
            let j = pixel.j;
            image[i][j] = new_color;
            for k in 0..4 {
                let i = i as i32 + di[k];
                let j = j as i32 + dj[k];
                if i < 0 {
                    continue;
                }
                let i = i as usize;
                if i >= n {
                    continue;
                }
                if j < 0 {
                    continue;
                }
                let j = j as usize;
                if j >= m {
                    continue;
                }
                if image[i][j] != c {
                    continue;
                }
                queue.push_back(Pixel { i, j })
            }
        }
        image
    }
}

#[test]
fn test() {
    let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
    let sr = 1;
    let sc = 1;
    let new_color = 2;
    let new_image = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
    assert_eq!(Solution::flood_fill(image, sr, sc, new_color), new_image);
}
