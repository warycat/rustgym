#[derive(Default)]
struct ProductOfNumbers {
    prefix: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        let prefix = vec![1];
        ProductOfNumbers { prefix }
    }

    fn add(&mut self, num: i32) {
        if num > 0 {
            let prev = self.prefix[self.prefix.len() - 1];
            self.prefix.push(prev * num);
        } else {
            self.prefix = vec![1];
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        let n = self.prefix.len();
        if k >= n {
            0
        } else {
            self.prefix[n - 1] / self.prefix[n - 1 - k]
        }
    }
}

#[test]
fn test() {
    let mut obj = ProductOfNumbers::new();
    obj.add(3);
    obj.add(0);
    obj.add(2);
    obj.add(5);
    obj.add(4);
    assert_eq!(obj.get_product(2), 20);
    assert_eq!(obj.get_product(3), 40);
    assert_eq!(obj.get_product(4), 0);
    obj.add(8);
    assert_eq!(obj.get_product(2), 32);
}
