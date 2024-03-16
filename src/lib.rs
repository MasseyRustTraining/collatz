//! [Collatz Sequence
//! exercise](https://google.github.io/comprehensive-rust/control-flow-basics/exercise.html)
//! from Comprehensive Rust.

/// Returns [Some] number of steps (length - 1) of the [Collatz
/// Sequence](https://en.wikipedia.org/wiki/Collatz_conjecture)
/// starting at `x`; if `x` is 0, instead return [None].
///
/// # Examples
///
/// ```
/// # use collatz::collatz_length;
/// assert!(collatz_length(3) == Some(7));
/// ```
pub fn collatz_length(mut x: u64) -> Option<usize> {
    if x == 0 {
        return None;
    }
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
    Some(n)
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_1(c: u64, v: usize) {
        assert!(collatz_length(c).unwrap() == v);
    }

    #[test]
    fn test_collatz_length() {
        assert!(collatz_length(0) == None);
        test_1(1, 0);
        assert!(collatz_length(3).unwrap() == 7);
        // From https://en.wikipedia.org/wiki/Collatz_conjecture
        assert!(collatz_length(670_617_279).unwrap() == 986);
        assert!(collatz_length(989_345_275_647).unwrap() == 1348);
    }
}
