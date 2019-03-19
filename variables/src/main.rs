fn main() {
    for mut n in 1..10 {
        println!("The {} fib number is {}", n, fibonacci(n));
        n = n + 1;
        println!("{}", n);
    }
}

fn fibonacci(n: isize) -> isize {
    match n {
        1 => 1,
        2 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2)
    }
}