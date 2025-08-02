fn main() {
    let mut s = String::from("hello");
   
    let r1 = &s;     // read-only borrow
    let r2 = &s;     // another read-only borrow - totally fine
   
    // let r3 = &mut s;  // This would be a problem!
   
    println!("{} and {}", r1, r2);
   
    let r3 = &mut s;  // Now this works!
    println!("{}", r3);
}
