struct Solution;

impl Solution {
    fn max_boxes_in_warehouse(mut boxes: Vec<i32>, warehouse: Vec<i32>) -> i32 {
        boxes.sort_unstable();
        boxes.reverse();
        let n = warehouse.len();
        let m = boxes.len();
        let mut i = 0;
        let mut j = 0;
        while i < m && j < n {
            if boxes[i] <= warehouse[j] {
                j += 1;
            }
            i += 1;
        }
        j as i32
    }
}

#[test]
fn test() {
    let boxes = vec![4, 3, 4, 1];
    let warehouse = vec![5, 3, 3, 4, 1];
    let res = 3;
    assert_eq!(Solution::max_boxes_in_warehouse(boxes, warehouse), res);
    let boxes = vec![1, 2, 2, 3, 4];
    let warehouse = vec![3, 4, 1, 2];
    let res = 3;
    assert_eq!(Solution::max_boxes_in_warehouse(boxes, warehouse), res);
    let boxes = vec![1, 2, 3];
    let warehouse = vec![1, 2, 3, 4];
    let res = 1;
    assert_eq!(Solution::max_boxes_in_warehouse(boxes, warehouse), res);
    let boxes = vec![4, 5, 6];
    let warehouse = vec![3, 3, 3, 3, 3];
    let res = 0;
    assert_eq!(Solution::max_boxes_in_warehouse(boxes, warehouse), res);
}
