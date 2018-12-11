use std::thread;
use std::time::Duration;

fn recur(current: u32) -> u32 {
    if current == 0 {
        // Sleep for 5 seconds so the state of RAM use can be observed
        thread::sleep(Duration::from_secs(5));
    }
    return if current == 0 {
        1
    } else {
        recur(current - 1)
    }
}

fn main() {
    println!("{}", recur(10_000_000));
}