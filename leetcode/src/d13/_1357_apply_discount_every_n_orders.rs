use std::collections::HashMap;

struct Cashier {
    n: usize,
    index: usize,
    discount: f64,
    inventory: HashMap<i32, f64>,
}

impl Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let n = n as usize;
        let index = 0;
        let discount = (100 - discount) as f64 / 100.0;
        let mut inventory: HashMap<i32, f64> = HashMap::new();
        for (id, price) in products.into_iter().zip(prices.into_iter()) {
            inventory.insert(id, price as f64);
        }
        Cashier {
            n,
            index,
            discount,
            inventory,
        }
    }
    fn get_bill(&mut self, products: Vec<i32>, amount: Vec<i32>) -> f64 {
        let mut res = 0.0;
        for (id, amount) in products.into_iter().zip(amount.into_iter()) {
            res += self.inventory[&id] * amount as f64;
        }
        self.index += 1;
        if self.index == self.n {
            self.index = 0;
            res * self.discount
        } else {
            res
        }
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let mut cashier = Cashier::new(
        3,
        50,
        vec![1, 2, 3, 4, 5, 6, 7],
        vec![100, 200, 300, 400, 300, 200, 100],
    );
    assert_approx_eq!(cashier.get_bill(vec![1, 2], vec![1, 2]), 500.0);
    assert_approx_eq!(cashier.get_bill(vec![3, 7], vec![10, 10]), 4000.0);
    assert_approx_eq!(
        cashier.get_bill(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 1, 1, 1, 1, 1, 1]),
        800.0
    );
    assert_approx_eq!(cashier.get_bill(vec![4], vec![10]), 4000.0);
    assert_approx_eq!(cashier.get_bill(vec![7, 3], vec![10, 10]), 4000.0);
    assert_approx_eq!(
        cashier.get_bill(vec![7, 5, 3, 1, 6, 4, 2], vec![10, 10, 10, 9, 9, 9, 7]),
        7350.0
    );
    assert_approx_eq!(cashier.get_bill(vec![2, 3, 5], vec![5, 3, 2]), 2500.0);
}
