use std::ops::Deref;

struct MyBox<T> {
    data: T,
}

impl<T> MyBox<T> {
    fn new(data: T) -> MyBox<T> {
        MyBox { data }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox!");
    }
}

fn main() {
    let my_box = MyBox::new(String::from("Custom smart pointer"));
    
    // Deref coercion allows us to use it like a regular reference
    println!("Length: {}", my_box.len());
    
    // Drop will be called automatically when my_box goes out of scope
}
