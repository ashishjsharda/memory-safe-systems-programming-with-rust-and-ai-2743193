use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }
}

fn main() {
    let parent = Node::new(1);
    let child = Node::new(2);
    
    // Child holds a strong reference to parent would create a cycle
    // Instead, use weak reference
    *child.parent.borrow_mut() = Rc::downgrade(&parent);
    
    // Clone the child reference before moving it
    let child_ref = Rc::clone(&child);
    parent.children.borrow_mut().push(child);
    
    println!("Parent strong count: {}", Rc::strong_count(&parent));
    
    // Fix: Extract the weak reference first, then upgrade it
    let weak_parent = child_ref.parent.borrow().clone();
    if let Some(parent_ref) = weak_parent.upgrade() {
        println!("Child's parent value: {}", parent_ref.value);
    }
}
