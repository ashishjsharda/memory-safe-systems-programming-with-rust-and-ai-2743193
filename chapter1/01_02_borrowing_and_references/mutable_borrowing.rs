fn main() {
    let mut s = String::from("hello");
   
    // We're borrowing s mutably here
    change(&mut s);
   
    println!("Modified string: {}", s);  // Prints "hello, world!"
}


fn change(s: &mut String) {
    s.push_str(", world!");
}
