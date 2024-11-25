use std::usize;

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

pub trait LinkedList<T> {
    fn len(&self) -> &usize;
    fn insert(&mut self, val: T, pos: &usize) -> bool;
    fn add(&mut self, val: T);
    // fn remove(&mut self, pos: &usize) -> Option<T>;
    fn contains(&self, val: T) -> bool;
}

// impl<T> Node<T> {
//     fn new(val: T) -> Self {
//         Self { val, next: None }
//     }
// }

macro_rules! node {
    ($val: expr) => {
        Some(Box::new(Node { val: $val, next: None }))
    };
    ($val: expr, $next: expr) => {
        Some(Box::new(Node { val: $val, next: $next }))
    }
}

pub mod linked_list;

pub use linked_list::SinglyLinkList;
