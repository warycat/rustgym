struct MyCircularQueue {
    k: usize,
    start: usize,
    end: usize,
    data: Vec<i32>,
    count: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        let start = 0;
        let end = 0;
        let k = k as usize;
        let data = vec![0; k];
        let count = 0;
        MyCircularQueue {
            k,
            start,
            end,
            data,
            count,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.count == self.k {
            false
        } else {
            self.count += 1;
            self.data[self.end] = value;
            self.end = (self.end + 1) % self.k;
            true
        }
    }

    fn de_queue(&mut self) -> bool {
        if self.count == 0 {
            false
        } else {
            self.count -= 1;
            self.start = (self.start + 1) % self.k;
            true
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.start]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[(self.end + self.k - 1) % self.k]
        }
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn is_full(&self) -> bool {
        self.count == self.k
    }
}

#[test]
fn test() {
    let mut queue = MyCircularQueue::new(3);
    assert_eq!(queue.en_queue(1), true);
    assert_eq!(queue.en_queue(2), true);
    assert_eq!(queue.en_queue(3), true);
    assert_eq!(queue.en_queue(4), false);
    assert_eq!(queue.rear(), 3);
    assert_eq!(queue.is_full(), true);
    assert_eq!(queue.de_queue(), true);
    assert_eq!(queue.en_queue(4), true);
    assert_eq!(queue.rear(), 4);
}
