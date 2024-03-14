fn collatz_length(x: i32) -> u32 {
    if x == 1 {
        1
    } else if x & 1 == 1 {
        // x is odd.
        1 + collatz_length(3 * x + 1)
    } else {
        // x is even.
        1 + collatz_length(x / 2)
    }
}

fn main() {
    let start = std::env::args().nth(1).unwrap().parse().unwrap();
    println!("{}", collatz_length(start));
}
