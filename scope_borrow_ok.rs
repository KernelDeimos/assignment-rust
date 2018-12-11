fn main() {
    let mut modify_me = String::from("Hey!");
    let test_closure = |borrow_and_modify_me: &mut String| {
        borrow_and_modify_me.push_str(" What's going on?");
    };

    // Scope limiter is required here because in Rust you cannot have a mutable
    // reference and an immutable reference simultaneously, which prevents
    // the println! macro from borrowing modify_me as immutable when
    // ref_modify_me is in scope.
    {
        let ref_modify_me = &mut modify_me;

        test_closure(ref_modify_me);
    }
    println!("{}", modify_me);
}
