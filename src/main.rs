//! [Collatz Sequence
//! exercise](https://google.github.io/comprehensive-rust/control-flow-basics/exercise.html)
//! from Comprehensive Rust.

/// Returns the length of the [Collatz
/// Sequence](https://en.wikipedia.org/wiki/Collatz_conjecture)
/// starting at `x`.
fn collatz_length(mut x: i32) -> u32 {
    let mut n = 1;
    while x != 1 {
        if x & 1 == 1 {
            // x is odd.
            x = 3 * x + 1;
        } else {
            // x is even.
            x = x / 2;
        }
        n += 1;
    }
    n
}

fn main() {
    let start = std::env::args().nth(1).unwrap().parse().unwrap();
    println!("{}", collatz_length(start));
}
