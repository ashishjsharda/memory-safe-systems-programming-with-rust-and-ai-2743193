fn main() {
    // Creating a String because we need a growable, heap-allocated string. Rust's String is fancy like that!
    let s1 = String::from("hello");
   
    // Heads-up: s1 is *giving up* its ownership here. s2 takes over, no cloning involved. Rust's move semantics at work!
    let s2 = s1;
   
    // If you try to use s1 here, Rust will yell at you. It's gone, kaput! Uncomment the line below to see the chaos.
    // println!("{}", s1);  // Nope, s1's value was moved. Compiler error incoming!
   
    // s2 is the new boss of "hello". This line works like a charm.
    println!("{}", s2);
}
