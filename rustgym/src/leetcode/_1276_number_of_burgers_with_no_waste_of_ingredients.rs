struct Solution;

impl Solution {
    fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        if tomato_slices - 2 * cheese_slices < 0 {
            return vec![];
        }
        if (tomato_slices - 2 * cheese_slices) % 2 != 0 {
            return vec![];
        }
        if cheese_slices * 4 - tomato_slices < 0 {
            return vec![];
        }
        if (cheese_slices * 4 - tomato_slices) % 2 != 0 {
            return vec![];
        }
        let x = (tomato_slices - 2 * cheese_slices) / 2;
        let y = (cheese_slices * 4 - tomato_slices) / 2;
        vec![x, y]
    }
}

#[test]
fn test() {
    let tomato_slices = 16;
    let cheese_slices = 7;
    let res = vec![1, 6];
    assert_eq!(Solution::num_of_burgers(tomato_slices, cheese_slices), res);
    let tomato_slices = 17;
    let cheese_slices = 4;
    let res: Vec<i32> = vec![];
    assert_eq!(Solution::num_of_burgers(tomato_slices, cheese_slices), res);
    let tomato_slices = 4;
    let cheese_slices = 17;
    let res: Vec<i32> = vec![];
    assert_eq!(Solution::num_of_burgers(tomato_slices, cheese_slices), res);
    let tomato_slices = 0;
    let cheese_slices = 0;
    let res = vec![0, 0];
    assert_eq!(Solution::num_of_burgers(tomato_slices, cheese_slices), res);
}
