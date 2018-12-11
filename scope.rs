fn main() {
    let modifier = 2;
    let test_closure = |num| {
        num + modifier
    };
    println!("{}", test_closure(5));
}
