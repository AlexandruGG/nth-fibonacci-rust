use std::{io, process};
fn main() {
    const MAX_RANK: i32 = 185;

    let fib_rank: i32 = read_input("Which Fibonacci rank do you want to compute?")
        .trim()
        .parse()
        .unwrap();

    if fib_rank > MAX_RANK {
        println!("Max Fibonnaci rank supported is 185.");
        process::exit(1)
    }

    let fib_number = nth_fib(fib_rank);
    println!("The rank {} Fibonacci number is {}.", fib_rank, fib_number);
}

fn read_input(message: &str) -> String {
    println!("{}", message);

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    user_input
}

fn nth_fib(n: i32) -> u128 {
    if n == 0 {
        return 0;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }

    b
}
