fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    for i in 0..10 {
        println!("fib({}) = {}", i, fib(i));
    }
}
