/// Determine the length of the collatz sequence beginning at `n`.
// if n(i) = 1 then terminate
// if n(i) % 2 then n(i+1) = n(i) / 2
// if n(i) % 1 then n(i+1) = 3 * n(i) + 1

fn collatz_length(mut n: i32) -> u32 {
    let mut i = 1;

    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        i += 1
    }

    i
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

fn main() {
    println!("Length: {}", collatz_length(11));
}
