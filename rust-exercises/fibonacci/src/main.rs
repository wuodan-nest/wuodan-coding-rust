// fibonacci series
// initialize the two starting numbers 0,1
// add 0+1 to get the second

fn fib(n: u32) -> u32 {
    if n == 0 {
        // The base case.
        return 0;
    } else if n == 1 {
	// base case
	return 1;
    } else {
        // The recursive case.
	return fib(n-1) + fib(n-2);
    }
}

fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}
