struct Solution;

trait MyClamp {
    fn my_clamp(self, min: i32, max: i32) -> i32;
}

impl MyClamp for i32 {
    fn my_clamp(self, min: i32, max: i32) -> i32 {
        self.max(min).min(max)
    }
}

impl Solution {
    fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let dx = x_center.my_clamp(x1, x2) - x_center;
        let dy = y_center.my_clamp(y1, y2) - y_center;
        dx * dx + dy * dy <= radius * radius
    }
}

#[test]
fn test() {
    let radius = 1;
    let x_center = 0;
    let y_center = 0;
    let x1 = 1;
    let y1 = -1;
    let x2 = 3;
    let y2 = 1;
    let res = true;
    assert_eq!(
        Solution::check_overlap(radius, x_center, y_center, x1, y1, x2, y2),
        res
    );
    let radius = 1;
    let x_center = 0;
    let y_center = 0;
    let x1 = -1;
    let y1 = 0;
    let x2 = 0;
    let y2 = 1;
    let res = true;
    assert_eq!(
        Solution::check_overlap(radius, x_center, y_center, x1, y1, x2, y2),
        res
    );
    let radius = 1;
    let x_center = 1;
    let y_center = 1;
    let x1 = -3;
    let y1 = -3;
    let x2 = 3;
    let y2 = 3;
    let res = true;
    assert_eq!(
        Solution::check_overlap(radius, x_center, y_center, x1, y1, x2, y2),
        res
    );
    let radius = 1;
    let x_center = 1;
    let y_center = 1;
    let x1 = 1;
    let y1 = -3;
    let x2 = 2;
    let y2 = -1;
    let res = false;
    assert_eq!(
        Solution::check_overlap(radius, x_center, y_center, x1, y1, x2, y2),
        res
    );
}
