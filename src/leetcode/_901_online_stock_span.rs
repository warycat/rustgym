#[derive(Default)]
struct StockSpanner {
    stack: Vec<(i32, usize)>,
}

impl StockSpanner {
    fn new() -> Self {
        StockSpanner::default()
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut res = 1;
        while let Some(top) = self.stack.pop() {
            if top.0 > price {
                self.stack.push(top);
                break;
            } else {
                res += top.1;
            }
        }
        self.stack.push((price, res));
        res as i32
    }
}

#[test]
fn test() {
    let mut obj = StockSpanner::new();
    assert_eq!(obj.next(100), 1);
    assert_eq!(obj.next(80), 1);
    assert_eq!(obj.next(60), 1);
    assert_eq!(obj.next(70), 2);
    assert_eq!(obj.next(60), 1);
    assert_eq!(obj.next(75), 4);
    assert_eq!(obj.next(85), 6);
}
