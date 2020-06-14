struct SubrectangleQueries {
    rectangle: Vec<Vec<i32>>,
    n: usize,
    m: usize,
}

impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        let n = rectangle.len();
        let m = rectangle[0].len();
        SubrectangleQueries { rectangle, n, m }
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        let r1 = row1 as usize;
        let c1 = col1 as usize;
        let r2 = row2 as usize;
        let c2 = col2 as usize;
        for i in r1..=r2 {
            for j in c1..=c2 {
                self.rectangle[i][j] = new_value;
            }
        }
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        self.rectangle[row as usize][col as usize]
    }
}

#[test]
fn test() {
    let rectangle = vec_vec_i32![[1, 2, 1], [4, 3, 4], [3, 2, 1], [1, 1, 1]];
    let mut obj = SubrectangleQueries::new(rectangle);
    assert_eq!(obj.get_value(0, 2), 1);
    obj.update_subrectangle(0, 0, 3, 2, 5);
    assert_eq!(obj.get_value(0, 2), 5);
}
