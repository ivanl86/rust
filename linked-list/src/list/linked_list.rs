use super::{LinkedList, Node};

pub struct SinglyLinkList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> SinglyLinkList<T> {
    pub fn new() -> Self {
        Self { head: None, size: 0 }
    }
}

impl<T> LinkedList<T> for SinglyLinkList<T> where T: PartialEq {
    fn len(&self) -> &usize {
        &self.size
    }

    fn add(&mut self, val: T) {
        let mut curr: &mut Option<Box<Node<T>>> = &mut self.head;

        while let Some(ref mut node) = curr {
            curr = &mut node.next
        }
        *curr = node!(val);
        self.size += 1;
    }

    fn contains(&self, val: T) -> bool {
        let mut curr = &self.head;
        
        while let Some(node) = curr {
            if node.val == val {
                return true;
            }
            curr = &node.next;
        }
        false
    }

    // list [1, 2, 3]
    // pos = idx 1
    // curr = idx 0
    // prv = idx 0
    // curr = idx 1

    fn insert(&mut self, val: T, pos: &usize) -> bool {
        if *pos >= self.size {
            return false;
        }
        let mut curr: &mut Option<Box<Node<T>>> = &mut self.head;
        let mut count: usize = 0;

        while count < *pos {
            if let Some(ref mut node) = curr {
                // prv = &node;
                curr = &mut node.next;
                count += 1;
            } else {
                break;
            }
        }

        *curr = node!(val, curr.take());
        // *curr = Some(Box::new(Node { val, next: curr.take() }));
        self.size += 1;
        true
    }

    // fn remove(&mut self, pos: &usize) -> Option<T> {
    //     Some(&self.head.unwrap().val)
    // }
}
