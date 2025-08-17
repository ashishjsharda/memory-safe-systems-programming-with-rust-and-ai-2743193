use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // RefCell allows mutation even when the RefCell itself is immutable
    let data = RefCell::new(42);
   
    // Borrow mutably at runtime
    {
        let mut borrowed = data.borrow_mut();
        *borrowed += 10;
        println!("Modified value: {}", *borrowed);
    } // mutable borrow ends here
   
    // Now we can borrow immutably
    let borrowed = data.borrow();
    println!("Current value: {}", *borrowed);
   
    // RefCell enforces borrowing rules at runtime
    // This would panic if we tried to borrow mutably while immutably borrowed
   
    // Demonstrate the common Rc<RefCell<T>> pattern
    shared_mutable_example();
}

// Common pattern: Rc<RefCell<T>> for shared mutable data
fn shared_mutable_example() {
    println!("\n--- Rc<RefCell<T>> Example ---");
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));
    let reference1 = Rc::clone(&shared_data);
    let reference2 = Rc::clone(&shared_data);
    
    // Both references can mutate the same data
    reference1.borrow_mut().push(4);
    reference2.borrow_mut().push(5);
    
    println!("Final vector: {:?}", shared_data.borrow());
}
