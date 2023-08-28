fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    let mut prev_prev = 0;
    let mut prev = 1;
    let mut current = 0;

    for _ in 2..=n { // Range is 2 to n
        current = prev + prev_prev;
        prev_prev = prev;
        prev = current;
    }

    current
}

fn main() {
    let n = 8; // Change the value on the right to get the n-th Fibonacci number
    println!("The {}-th Fibonacci number is: {}", n, fibonacci(n));
}
