use crate::linklist::{DoubleNode, double_pointer};

#[derive(Debug)]
pub struct DoubleLinklist {
    head: double_pointer,
    tail: double_pointer,
}

impl DoubleLinklist {
    pub fn new() -> Self {
        DoubleLinklist {
            head: None,
            tail: None,
        }
    }

    pub fn add(&mut self, element: i32) {
        let new_head = DoubleNode::new(element);

        match self.head.take() {
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn remove(&mut self) -> Option<i32> {
        if self.head.is_none() {
            println!("List is empty so we can not remove");
            None
        } else {
            let removed_val = self.head.as_ref().unwrap().borrow().element;
            self
                .head
                .take()
                .map(|old_head| match old_head.borrow_mut().next.take() {
                    None => {
                        self.tail = None;
                        println!("List is empty after removal");
                        None
                    }
                    Some(new_head) => {
                        new_head.borrow_mut().prev = None;
                        self.head = Some(new_head);
                        self.head.clone()
                    }
                });
            Some(removed_val)
        }
    }

    pub fn print(&self) {
        let mut traversal = self.head.clone();
        while !traversal.is_none() {
            println!("{}", traversal.as_ref().unwrap().borrow().element);
            traversal = traversal.unwrap().borrow().next.clone();
        }
    }
}