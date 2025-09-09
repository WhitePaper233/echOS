#![no_std]
#![no_main]

#[macro_use]
extern crate user;

fn fib(n: i64) -> i64 {
    if n == 0 || n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

#[unsafe(no_mangle)]
fn main() -> i32 {
    let result = fib(10);
    println!("[app_2] The 10th fibonacci sequence item is {}", result);

    return 0;
}
