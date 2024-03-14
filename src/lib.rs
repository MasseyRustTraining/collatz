//! [Collatz Sequence
//! exercise](https://google.github.io/comprehensive-rust/control-flow-basics/exercise.html)
//! from Comprehensive Rust.

/// Returns the number of steps (length - 1) of the [Collatz
/// Sequence](https://en.wikipedia.org/wiki/Collatz_conjecture)
/// starting at `x`.
///
/// # Examples
///
/// ```
/// assert!(collatz_length(3) == 8);
/// ```
pub fn collatz_length(mut x: u64) -> usize {
    let mut n = 0;
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
    assert!(collatz_length(1) == 0);
    assert!(collatz_length(3) == 7);
    // From https://en.wikipedia.org/wiki/Collatz_conjecture
    assert!(collatz_length(670_617_279) == 986);
    assert!(collatz_length(989_345_275_647) == 1348);
}
