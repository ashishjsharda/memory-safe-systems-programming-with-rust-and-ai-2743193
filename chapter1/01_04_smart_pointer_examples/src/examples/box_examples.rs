fn main() {
    // Storing a simple value on the heap
    let boxed_number = Box::new(42);
    println!("Boxed number: {}", boxed_number);
   
    // Box is particularly useful for large data structures
    let large_array = Box::new([0; 10000]); // Ten Thousand integers on the heap
    println!("First element: {}", large_array[0]);
   
    // Box enables recursive data structures
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
   
    // Function to calculate the sum of all elements in the list
    fn sum_list(list: &List) -> i32 {
        match list {
            List::Cons(value, next) => value + sum_list(next),
            List::Nil => 0,
        }
    }
   
    // Function to get the length of the list
    fn list_length(list: &List) -> usize {
        match list {
            List::Cons(_, next) => 1 + list_length(next),
            List::Nil => 0,
        }
    }
   
    let list = List::Cons(1,
        Box::new(List::Cons(2,
            Box::new(List::Cons(3,
                Box::new(List::Nil))))));
   
    println!("Linked list: {:?}", list);
    println!("Sum of list elements: {}", sum_list(&list));
    println!("Length of list: {}", list_length(&list));
}
