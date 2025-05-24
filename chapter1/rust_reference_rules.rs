fn main() {
    let mut s = String::from("hello");
    let r1 = &s;     // Immutable reference
    let r2 = &s;     // Another immutable reference - this is fine
    // let r3 = &mut s;  // Error: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{} and {}", r1, r2);
    // After this point, r1 and r2 are no longer used, so...
    let r3 = &mut s;  // This is now allowed!
    println!("{}", r3);
}
