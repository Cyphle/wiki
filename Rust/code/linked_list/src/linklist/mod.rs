mod node;
mod single_linklist;
mod double_linklist;

use std::cell::RefCell;
use std::rc::Rc;

pub use crate::linklist::node::SingleNode;
pub use crate::linklist::single_linklist::SingleLinklist;

pub use crate::linklist::node::DoubleNode;
pub use crate::linklist::double_linklist::DoubleLinklist;

pub type pointer = Option<Box<SingleNode>>;
pub type double_pointer = Option<Rc<RefCell<DoubleNode>>>;
