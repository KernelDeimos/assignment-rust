fn main() {
    let mut modify_me = String::from("Hey!");
    let test_closure = |num| {
        modify_me.push_str(" What's going on?");
        num + 1
    };
    println!("{}", test_closure(5));
}
