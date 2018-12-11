fn main() {
    let modify_me = String::from("Hey!");
    let test_closure = |mut own_and_modify_me: String| {
        own_and_modify_me.push_str(" What's going on?");
        own_and_modify_me
    };
    println!("{}", test_closure(modify_me));
    
    // modify_me can no longer be used at this point because ownership was
    // transferred to test_closure
}
