fn main() {
    taking();
    takeing_and_giving();
    references_basics();
}

// {{## BEGIN taking ##}}
fn taking() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    println!("{}", s);            // compile error!
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.
// {{## END taking ##}}

// {{## BEGIN taking-and-giving ##}}
fn takeing_and_giving() {
    let s1 = gives_ownership();         // move
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  // move into takes_and_gives_back, which also
                                        // moves its return value into s3
}
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string                         // move out to caller
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string                            // moves out to caller
}
// {{## END taking-and-giving ##}}

// {{## BEGIN refs ##}}
fn references_basics() {
    let s = String::from("hello");
    let len = calculate_length(&s);        // notice the "&" in the call
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {  // notice the "&" in the declaration
    //s.push_str(" I am in you!");          // compile error!
    s.len()
}
// {{## END refs ##}}

// {{## BEGIN mutable-refs ##}}
fn mutable_references_basics() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
// {{## END mutable-refs ##}}

