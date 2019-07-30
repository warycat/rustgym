struct Vector2D {
    array: Vec<i32>,
    count: usize,
}

impl Vector2D {
    fn new(v: Vec<Vec<i32>>) -> Self {
        let mut array = vec![];
        for row in v {
            for col in row {
                array.push(col);
            }
        }
        Vector2D { array, count: 0 }
    }
    fn next(&mut self) -> i32 {
        let res = self.array[self.count];
        self.count += 1;
        res
    }
    fn has_next(&self) -> bool {
        self.count < self.array.len()
    }
}

#[test]
fn test() {
    let v = vec![vec![1, 2], vec![3], vec![4]];
    let mut obj = Vector2D::new(v);
    assert_eq!(obj.next(), 1);
    assert_eq!(obj.next(), 2);
    assert_eq!(obj.next(), 3);
    assert_eq!(obj.has_next(), true);
    assert_eq!(obj.has_next(), true);
    assert_eq!(obj.next(), 4);
    assert_eq!(obj.has_next(), false);
}
