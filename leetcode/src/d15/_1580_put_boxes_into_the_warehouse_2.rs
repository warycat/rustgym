struct Solution;

impl Solution {
    fn max_boxes_in_warehouse(mut boxes: Vec<i32>, warehouse: Vec<i32>) -> i32 {
        boxes.sort_unstable();
        let n = warehouse.len();
        let mut l = 0;
        let mut r = n - 1;
        let mut limit = warehouse[l].max(warehouse[r]);
        let mut res = 0;
        while l <= r {
            let taller = warehouse[l].max(warehouse[r]);
            while let Some(&max) = boxes.last() {
                if max > limit || max > taller {
                    boxes.pop();
                } else {
                    break;
                }
            }
            if boxes.pop().is_none() {
                break;
            }
            res += 1;
            if l == r {
                break;
            }
            if warehouse[l] <= warehouse[r] {
                limit = limit.min(warehouse[r]);
                r -= 1;
            } else {
                limit = limit.min(warehouse[l]);
                l += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let boxes = vec![1, 2, 2, 3, 4];
    let warehouse = vec![3, 4, 1, 2];
    let res = 4;
    assert_eq!(Solution::max_boxes_in_warehouse(boxes, warehouse), res);
    let boxes = vec![3, 5, 5, 2];
    let warehouse = vec![2, 1, 3, 4, 5];
    let res = 3;
    assert_eq!(Solution::max_boxes_in_warehouse(boxes, warehouse), res);
    let boxes = vec![1, 2, 3];
    let warehouse = vec![1, 2, 3, 4];
    let res = 3;
    assert_eq!(Solution::max_boxes_in_warehouse(boxes, warehouse), res);
    let boxes = vec![4, 5, 6];
    let warehouse = vec![3, 3, 3, 3, 3];
    let res = 0;
    assert_eq!(Solution::max_boxes_in_warehouse(boxes, warehouse), res);
}
