struct Solution;

impl Solution {
    fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let m = flowerbed.len();
        let mut sum = 0;
        for i in 0..m {
            if flowerbed[i] == 0 {
                if i == 0 {
                    sum += 1;
                    flowerbed[i] = 1;
                } else {
                    if flowerbed[i - 1] == 0 {
                        sum += 1;
                        flowerbed[i] = 1;
                    }
                }
            } else {
                if i > 0 && flowerbed[i - 1] == 1 {
                    sum -= 1;
                }
            }
        }
        sum >= n
    }
}

#[test]
fn test() {
    let flowerbed: Vec<i32> = vec![1, 0, 0, 0, 1];
    assert_eq!(Solution::can_place_flowers(flowerbed, 1), true);
    let flowerbed: Vec<i32> = vec![1, 0, 0, 0, 1];
    assert_eq!(Solution::can_place_flowers(flowerbed, 2), false);
}
