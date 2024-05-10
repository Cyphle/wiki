use std::cell::RefCell;
use std::rc::Rc;
use crate::linklist::{double_pointer, pointer};

#[derive(Debug)]
pub struct SingleNode {
    pub element: i32,
    pub next: pointer,
}

#[derive(Debug)]
pub struct DoubleNode {
    pub element: i32,
    pub next: double_pointer,
    pub prev: double_pointer,
}

impl DoubleNode {
    pub fn new(element: i32) -> Rc<RefCell<DoubleNode>> {
        Rc::new(RefCell::new(DoubleNode {
            element: element,
            next: None,
            prev: None,
        }))
    }
}