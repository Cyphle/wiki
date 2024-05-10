use crate::linklist::{SingleNode, pointer};

#[derive(Debug)]
pub struct SingleLinklist {
    head: pointer,
}

impl SingleLinklist {
    pub fn new() -> SingleLinklist {
        SingleLinklist { head: None }
    }

    pub fn add(&mut self, element: i32) {
        // This option does not work because we match self.head and replace it in same block of code
        // match self.head {
        //     Some(previous_head) => {
        //         let new_node = Some(Box::new(Node {
        //             element: element,
        //             next: Some(previous_head),
        //         }));
        //         self.head = new_node;
        //     }
        //     None => {
        //         let new_node = Some(Box::new(Node {
        //             element: element,
        //             next: None,
        //         }));
        //         self.head = new_node;
        //     }
        // }
        let previous_head = self.head.take();
        let new_node = Some(Box::new(SingleNode {
            element: element,
            next: previous_head,
        }));
        self.head = new_node;
    }

    pub fn remove(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            }
        }
    }

    pub fn print(&self) {
        let mut list_traversel = &self.head;
        while !list_traversel.is_none() {
            println!("{:?}", list_traversel.as_ref().unwrap().element);
            list_traversel = &list_traversel.as_ref().unwrap().next;
        }
    }
}