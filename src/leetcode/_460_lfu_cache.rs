use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node>>;
type Link = Option<NodeRef>;

struct Node {
    key: i32,
    value: i32,
    freq: usize,
    prev: Link,
    next: Link,
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        let freq = 1;
        let prev = None;
        let next = None;
        Node {
            key,
            value,
            freq,
            prev,
            next,
        }
    }
}

#[derive(Default)]
struct LinkedList {
    head: Link,
    tail: Link,
}

impl LinkedList {
    fn pop_front(&mut self) -> Link {
        if let Some(first) = self.head.take() {
            if let Some(second) = first.borrow_mut().next.take() {
                second.borrow_mut().prev = None;
                self.head = Some(second);
            } else {
                self.tail = None;
                self.head = None;
            }
            Some(first)
        } else {
            None
        }
    }

    fn push_back(&mut self, node_ref: NodeRef) {
        if let Some(last) = self.tail.take() {
            last.borrow_mut().next = Some(node_ref.clone());
            node_ref.borrow_mut().prev = Some(last);
        } else {
            self.head = Some(node_ref.clone());
        }
        self.tail = Some(node_ref);
    }

    fn is_empty(&self) -> bool {
        self.head.is_none() && self.tail.is_none()
    }
}

struct LFUCache {
    capacity: usize,
    count: usize,
    min_freq: RefCell<usize>,
    values: HashMap<i32, NodeRef>,
    freqs: RefCell<HashMap<usize, LinkedList>>,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        let count = 0;
        let min_freq = RefCell::new(0);
        let values = HashMap::new();
        let freqs = RefCell::new(HashMap::new());
        LFUCache {
            capacity,
            count,
            min_freq,
            values,
            freqs,
        }
    }

    fn min_freq(&self) -> usize {
        *self.min_freq.borrow()
    }

    fn set_min_freq(&self, freq: usize) {
        *self.min_freq.borrow_mut() = freq;
    }

    fn get(&self, key: i32) -> i32 {
        if self.capacity == 0 {
            return -1;
        }
        if let Some(node_ref) = self.values.get(&key) {
            let value = node_ref.borrow().value;
            self.update_freq(node_ref.clone());
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, val: i32) {
        if self.capacity == 0 {
            return;
        }
        if let Some(node_ref) = self.values.get(&key) {
            node_ref.borrow_mut().value = val;
            self.update_freq(node_ref.clone());
        } else {
            if self.count == self.capacity {
                let node_ref = self.pop_front_noderef(self.min_freq()).unwrap();
                self.values.remove(&node_ref.borrow().key);
            } else {
                self.count += 1;
            }
            let node_ref = Rc::new(RefCell::new(Node::new(key, val)));
            self.values.insert(key, node_ref.clone());
            self.freqs
                .borrow_mut()
                .entry(1)
                .or_default()
                .push_back(node_ref);
            self.set_min_freq(1);
        }
    }

    fn update_freq(&self, node_ref: NodeRef) {
        let freq = node_ref.borrow().freq;
        node_ref.borrow_mut().freq += 1;
        self.push_back_noderef(freq + 1, self.take_noderef(freq, node_ref));
        if freq == self.min_freq() && self.freqs.borrow_mut().entry(freq).or_default().is_empty() {
            self.set_min_freq(freq + 1);
        }
    }

    fn take_noderef(&self, freq: usize, node_ref: NodeRef) -> NodeRef {
        let mut freqs = self.freqs.borrow_mut();
        let linked_list = freqs.get_mut(&freq).unwrap();
        {
            let mut node = node_ref.borrow_mut();
            match (node.prev.take(), node.next.take()) {
                (Some(prev), Some(next)) => {
                    next.borrow_mut().prev = Some(prev.clone());
                    prev.borrow_mut().next = Some(next);
                }
                (None, Some(next)) => {
                    next.borrow_mut().prev = None;
                    linked_list.head = Some(next);
                }
                (Some(prev), None) => {
                    prev.borrow_mut().next = None;
                    linked_list.tail = Some(prev);
                }
                (None, None) => {
                    linked_list.head = None;
                    linked_list.tail = None;
                }
            }
        }
        node_ref
    }

    fn push_back_noderef(&self, freq: usize, node_ref: NodeRef) {
        let mut freqs = self.freqs.borrow_mut();
        let linked_list = freqs.entry(freq).or_default();
        linked_list.push_back(node_ref);
    }

    fn pop_front_noderef(&self, freq: usize) -> Link {
        if let Some(linked_list) = self.freqs.borrow_mut().get_mut(&freq) {
            linked_list.pop_front()
        } else {
            None
        }
    }
}

#[test]
fn test() {
    let mut cache = LFUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3);
    assert_eq!(cache.get(2), -1);
    assert_eq!(cache.get(3), 3);
    cache.put(4, 4);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);

    let mut cache = LFUCache::new(3);
    cache.put(1, 1);
    cache.put(2, 2);
    cache.put(3, 3);
    cache.put(4, 4);
    assert_eq!(cache.get(4), 4);
    assert_eq!(cache.get(3), 3);
}
