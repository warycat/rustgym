use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::rc::{Rc, Weak};

type NodeRef = Rc<RefCell<Node>>;
type Link = Option<NodeRef>;

#[derive(Debug, Clone)]
struct Element {
    key: i32,
    val: i32,
}

impl Element {
    fn new(key: i32, val: i32) -> Self {
        Element { key, val }
    }
}

#[derive(Debug)]
struct Node {
    element: Element,
    prev: Link,
    next: Link,
}

impl Node {
    fn new(element: Element) -> Self {
        Node {
            element,
            prev: None,
            next: None,
        }
    }
    fn into_ref(self) -> NodeRef {
        Rc::new(RefCell::new(self))
    }
}

#[derive(Default)]
struct List {
    head: Link,
    tail: Link,
}

impl List {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, new_node_ref: NodeRef) {
        new_node_ref.borrow_mut().prev = self.tail.clone();
        match self.tail.clone() {
            None => self.head = Some(new_node_ref.clone()),
            Some(last_node_ref) => last_node_ref.borrow_mut().next = Some(new_node_ref.clone()),
        }
        self.tail = Some(new_node_ref);
    }

    fn pop_front(&mut self) -> Option<Element> {
        self.head.clone().map(|node_ref| {
            let mut node = node_ref.borrow_mut();
            if let Some(next_node_ref) = node.next.take() {
                let mut next_node = next_node_ref.borrow_mut();
                next_node.prev = None;
                self.head = Some(next_node_ref.clone());
            } else {
                self.head = None;
                self.tail = None;
            }
            node.element.clone()
        })
    }

    fn unlink_node(&mut self, node_ref: NodeRef) -> Element {
        let mut node = node_ref.borrow_mut();
        let prev = node.prev.take();
        let next = node.next.take();
        match prev.clone() {
            Some(prev_node_ref) => prev_node_ref.borrow_mut().next = next.clone(),
            None => self.head = next.clone(),
        }
        match next {
            Some(next_node_ref) => next_node_ref.borrow_mut().prev = prev,
            None => self.tail = prev,
        }
        node.element.clone()
    }
    fn iter(&self) -> ListIter {
        ListIter::new(self.head.clone(), self.tail.clone())
    }
}

impl Debug for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl Drop for List {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

struct ListIter {
    head: Link,
    tail: Link,
}

impl ListIter {
    fn new(head: Link, tail: Link) -> Self {
        ListIter { head, tail }
    }
}

impl Iterator for ListIter {
    type Item = Element;
    fn next(&mut self) -> Option<Self::Item> {
        self.head.clone().map(|node_ref| {
            let node = node_ref.borrow();
            self.head = node.next.clone();
            node.element.clone()
        })
    }
}

#[derive(Debug)]
struct LRUCache {
    hash_map: HashMap<i32, Weak<RefCell<Node>>>,
    list: List,
    length: usize,
    capacity: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let hash_map: HashMap<i32, Weak<RefCell<Node>>> = HashMap::new();
        let list: List = List::new();
        let length = 0;
        let capacity = capacity as usize;
        LRUCache {
            hash_map,
            list,
            length,
            capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(element) = self.get_element(key) {
            element.val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, val: i32) {
        if self.put_element(key, val).is_none() {
            self.length += 1;
        }
        if self.length > self.capacity {
            self.pop_element();
            self.length -= 1;
        }
    }

    fn pop_element(&mut self) -> Option<Element> {
        if let Some(element) = self.list.pop_front() {
            self.hash_map.remove(&element.key);
            Some(element)
        } else {
            None
        }
    }

    fn put_element(&mut self, key: i32, val: i32) -> Option<Element> {
        if let Some(node_ref) = self.get_ref(key) {
            let old_element = self.list.unlink_node(node_ref);
            let new_element = Element::new(key, val);
            let new_node_ref = Node::new(new_element).into_ref();
            self.hash_map.insert(key, Rc::downgrade(&new_node_ref));
            self.list.push_back(new_node_ref);
            Some(old_element)
        } else {
            let new_element = Element::new(key, val);
            let new_node_ref = Node::new(new_element).into_ref();
            self.hash_map.insert(key, Rc::downgrade(&new_node_ref));
            self.list.push_back(new_node_ref);
            None
        }
    }

    fn get_element(&mut self, key: i32) -> Option<Element> {
        if let Some(node_ref) = self.get_ref(key) {
            let old_element = self.list.unlink_node(node_ref);
            let new_element = Element::new(old_element.key, old_element.val);
            let new_node_ref = Node::new(new_element).into_ref();
            self.hash_map.insert(key, Rc::downgrade(&new_node_ref));
            self.list.push_back(new_node_ref);
            Some(old_element)
        } else {
            None
        }
    }

    fn get_ref(&mut self, key: i32) -> Option<NodeRef> {
        if let Some(weak) = self.hash_map.remove(&key) {
            weak.upgrade()
        } else {
            None
        }
    }
}

#[test]
fn test() {
    let mut obj = LRUCache::new(2);
    assert_eq!(obj.get(2), -1);
    obj.put(2, 6);
    assert_eq!(obj.get(1), -1);
    obj.put(1, 5);
    obj.put(1, 2);
    assert_eq!(obj.get(1), 2);
    assert_eq!(obj.get(2), 6);

    let mut obj = LRUCache::new(3);
    obj.put(1, 1);
    obj.put(2, 2);
    obj.put(3, 3);
    obj.put(4, 4);
    assert_eq!(obj.get(4), 4);
    assert_eq!(obj.get(3), 3);
    assert_eq!(obj.get(2), 2);
    assert_eq!(obj.get(1), -1);
    obj.put(5, 5);
    assert_eq!(obj.get(1), -1);
    assert_eq!(obj.get(2), 2);
    assert_eq!(obj.get(3), 3);
    assert_eq!(obj.get(4), -1);
    assert_eq!(obj.get(5), 5);
}
