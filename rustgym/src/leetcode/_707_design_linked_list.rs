use rustgym_util::*;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct MyLinkedList {
    head: ListLink,
}

impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList { head: None }
    }

    fn get(&self, index: i32) -> i32 {
        if index < 0 {
            return -1;
        }
        let mut i = 0;
        let mut link: &ListLink = &self.head;
        while let Some(node) = link {
            if i == index {
                return node.val;
            }
            i += 1;
            link = &node.next;
        }
        -1
    }

    fn add_at_head(&mut self, val: i32) {
        self.head = ListLink::link(val, self.head.take());
    }

    fn add_at_tail(&mut self, val: i32) {
        let mut link: &mut ListLink = &mut self.head;
        while let Some(node) = link {
            link = &mut node.next;
        }
        *link = ListLink::link(val, None);
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index <= 0 {
            self.add_at_head(val);
        } else {
            let mut i = 0;
            let mut link: &mut ListLink = &mut self.head;
            while let Some(node) = link {
                if index == i + 1 {
                    node.next = ListLink::link(val, node.next.take());
                    return;
                } else {
                    link = &mut node.next;
                    i += 1;
                }
            }
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index < 0 {
            return;
        }
        let mut i = 0;
        let mut link: &mut ListLink = &mut self.head;
        loop {
            match link {
                None => {
                    return;
                }
                Some(node) if index == i => {
                    *link = node.next.take();
                    return;
                }
                Some(node) => {
                    link = &mut node.next;
                    i += 1;
                }
            }
        }
    }
}

#[test]
fn test() {
    let mut obj = MyLinkedList::new();
    obj.add_at_head(1);
    obj.add_at_tail(3);
    obj.add_at_index(1, 2);
    assert_eq!(obj.get(1), 2);
    obj.delete_at_index(1);
    assert_eq!(obj.get(1), 3);
    let mut obj = MyLinkedList::new();
    obj.add_at_head(1);
    obj.add_at_index(1, 2);
    assert_eq!(obj.get(1), 2);
}
