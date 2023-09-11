use std::io;

fn main() {
    println!("Enter a number for nth Fibonacci number: ");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = n
        .trim()
        .parse()
        .expect("Please type a number!");

    println!("The {}th Fibonacci number is {}", n, fib(n));
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
}
