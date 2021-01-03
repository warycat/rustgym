struct Solution;

impl Solution {
    fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut boxes: Vec<(i32, i32)> = box_types.into_iter().map(|v| (v[1], v[0])).collect();
        boxes.sort_unstable();
        let mut size = 0;
        let mut res = 0;
        while let Some((unit, k)) = boxes.pop() {
            if size + k < truck_size {
                res += unit * k;
                size += k;
            } else {
                let left = truck_size - size;
                res += left * unit;
                break;
            }
        }
        res
    }
}

#[test]
fn test() {
    let box_types = vec_vec_i32![[1, 3], [2, 2], [3, 1]];
    let truck_size = 4;
    let res = 8;
    assert_eq!(Solution::maximum_units(box_types, truck_size), res);
    let box_types = vec_vec_i32![[5, 10], [2, 5], [4, 7], [3, 9]];
    let truck_size = 10;
    let res = 91;
    assert_eq!(Solution::maximum_units(box_types, truck_size), res);
}
