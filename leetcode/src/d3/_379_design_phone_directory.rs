use std::collections::HashSet;

struct PhoneDirectory {
    available: HashSet<i32>,
}

impl PhoneDirectory {
    fn new(max_numbers: i32) -> Self {
        let available = (0..max_numbers).collect();
        PhoneDirectory { available }
    }

    fn get(&mut self) -> i32 {
        if let Some(&x) = self.available.iter().next() {
            self.available.remove(&x);
            x
        } else {
            -1
        }
    }

    fn check(&self, number: i32) -> bool {
        self.available.contains(&number)
    }

    fn release(&mut self, number: i32) {
        self.available.insert(number);
    }
}

#[test]
fn test() {
    let mut obj = PhoneDirectory::new(1);
    assert_eq!(obj.get(), 0);
    assert_eq!(obj.check(0), false);
    assert_eq!(obj.get(), -1);
    obj.release(2);
    assert_eq!(obj.check(2), true);
}
