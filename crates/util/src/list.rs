#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: ListLink,
}

#[macro_export]
macro_rules! list {
    () => {
        None
    };
    ($e:expr) => {
        ListNode::node($e, None)
    };
    ($e:expr, $($tail:tt)*) => {
        ListNode::node($e, list!($($tail)*))
    };
}

pub type ListLink = Option<Box<ListNode>>;

impl ListNode {
    pub fn node(val: i32, next: ListLink) -> ListLink {
        Some(Box::new(ListNode { val, next }))
    }
}
