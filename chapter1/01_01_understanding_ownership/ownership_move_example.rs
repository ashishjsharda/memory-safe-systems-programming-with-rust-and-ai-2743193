fn main() {
    // String is a more complex type than primitive types
    let s1 = String::from("hello");
   
    // This doesn't create a copy - it MOVES ownership
    let s2 = s1;
   
    // This would cause a compile error - s1 no longer owns the value
    // println!("{}", s1);  // Error: value borrowed after move
   
    // This works fine - s2 is the current owner
    println!("{}", s2);
}
