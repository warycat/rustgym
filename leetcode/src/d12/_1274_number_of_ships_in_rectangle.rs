struct Solution;

struct Sea {
    ships: Vec<Vec<i32>>,
}

impl Sea {
    fn new(ships: Vec<Vec<i32>>) -> Self {
        Sea { ships }
    }

    #[allow(non_snake_case)]
    fn hasShips(&self, top_right: Vec<i32>, bottom_left: Vec<i32>) -> bool {
        for ship in &self.ships {
            if ship.inside(&top_right, &bottom_left) {
                return true;
            }
        }
        false
    }
}

trait Inside {
    fn inside(&self, top_right: &[i32], bottom_left: &[i32]) -> bool;
}

impl Inside for Vec<i32> {
    fn inside(&self, top_right: &[i32], bottom_left: &[i32]) -> bool {
        let x = self[0];
        let y = self[1];
        let x1 = bottom_left[0];
        let y1 = bottom_left[1];
        let x2 = top_right[0];
        let y2 = top_right[1];
        x1 <= x && x <= x2 && y1 <= y && y <= y2
    }
}

impl Solution {
    fn count_ships(sea: &Sea, top_right: Vec<i32>, bottom_left: Vec<i32>) -> i32 {
        if !sea.hasShips(top_right.clone(), bottom_left.clone()) {
            0
        } else {
            let x1 = bottom_left[0];
            let y1 = bottom_left[1];
            let x2 = top_right[0];
            let y2 = top_right[1];
            let x = (x1 + x2) / 2;
            let y = (y1 + y2) / 2;
            if x1 == x2 && y1 == y2 {
                return 1;
            }
            if x1 == x2 {
                let top = Self::count_ships(sea, vec![x1, y2], vec![x1, y + 1]);
                let bottom = Self::count_ships(sea, vec![x1, y1], vec![x1, y]);
                return top + bottom;
            }
            if y1 == y2 {
                let left = Self::count_ships(sea, vec![x, y1], vec![x1, y1]);
                let right = Self::count_ships(sea, vec![x2, y1], vec![x + 1, y1]);
                return left + right;
            }
            let a = Self::count_ships(sea, vec![x2, y2], vec![x + 1, y + 1]);
            let b = Self::count_ships(sea, vec![x, y2], vec![x1, y + 1]);
            let c = Self::count_ships(sea, vec![x, y], vec![x1, y1]);
            let d = Self::count_ships(sea, vec![x2, y], vec![x + 1, y1]);
            a + b + c + d
        }
    }
}

#[test]
fn test() {
    let ships = vec_vec_i32![[1, 1], [2, 2], [3, 3], [5, 5]];
    let top_right = vec![4, 4];
    let bottom_left = vec![0, 0];
    let sea = Sea::new(ships);
    let res = 3;
    assert_eq!(Solution::count_ships(&sea, top_right, bottom_left), res);
}
