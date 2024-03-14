use collatz::collatz_length;

fn main() {
    let start = std::env::args().nth(1).unwrap().parse().unwrap();
    println!("{}", collatz_length(start));
}
