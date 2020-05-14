pub struct MyCircularDeque {
    k: usize,
    start: usize,
    end: usize,
    data: Vec<i32>,
    count: usize,
}

impl MyCircularDeque {
    pub fn new(k: i32) -> Self {
        let start = 0;
        let end = 0;
        let k = k as usize;
        let data = vec![0; k];
        let count = 0;
        MyCircularDeque {
            k,
            start,
            end,
            data,
            count,
        }
    }
    pub fn insert_front(&mut self, value: i32) -> bool {
        if self.count == self.k {
            false
        } else {
            self.count += 1;
            self.start = (self.start + self.k - 1) % self.k;
            self.data[self.start] = value;
            true
        }
    }

    pub fn insert_last(&mut self, value: i32) -> bool {
        if self.count == self.k {
            false
        } else {
            self.count += 1;
            self.data[self.end] = value;
            self.end = (self.end + 1) % self.k;
            true
        }
    }

    pub fn delete_front(&mut self) -> bool {
        if self.count == 0 {
            false
        } else {
            self.count -= 1;
            self.start = (self.start + 1) % self.k;
            true
        }
    }

    pub fn delete_last(&mut self) -> bool {
        if self.count == 0 {
            false
        } else {
            self.count -= 1;
            self.end = (self.end + self.k - 1) % self.k;
            true
        }
    }

    pub fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.start]
        }
    }

    pub fn get_rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[(self.end + self.k - 1) % self.k]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn is_full(&self) -> bool {
        self.count == self.k
    }
}

#[test]
fn test() {
    let mut queue = MyCircularDeque::new(3);
    assert_eq!(queue.insert_last(1), true);
    assert_eq!(queue.insert_last(2), true);
    assert_eq!(queue.insert_front(3), true);
    assert_eq!(queue.insert_front(4), false);
    assert_eq!(queue.get_rear(), 2);
    assert_eq!(queue.is_full(), true);
    assert_eq!(queue.delete_last(), true);
    assert_eq!(queue.insert_front(4), true);
    assert_eq!(queue.get_front(), 4);
}
