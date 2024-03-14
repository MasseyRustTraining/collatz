//! [Collatz Sequence
//! exercise](https://google.github.io/comprehensive-rust/control-flow-basics/exercise.html)
//! from Comprehensive Rust.

/// Returns the length of the [Collatz
/// Sequence](https://en.wikipedia.org/wiki/Collatz_conjecture)
/// starting at `x`.
///
/// # Examples
///
/// ```
/// assert!(collatz_length(3) == 8);
/// ```
fn collatz_length(mut x: u64) -> usize {
    let mut n = 1;
    while x != 1 {
        match x & 1 == 1 {
            true => {
                // x is odd.
                x = 3 * x + 1;
            }
            false => {
                // x is even.
                x /= 2;
            }
        }
        n += 1;
    }
    n
}

#[test]
fn test_collatz_length() {
    assert!(collatz_length(1) == 1);
}

fn main() {
    let start = std::env::args().nth(1).unwrap().parse().unwrap();
    println!("{}", collatz_length(start));
}
