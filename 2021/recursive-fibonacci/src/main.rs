// fn+1 = fn + fn-1
// starting with f(1) = 1 and f(2) = 1

const FIB_ZERO: u64 = 1;
const FIB_ONE: u64 = 1;

fn fib(n: u64) -> u64 {
    if n == 0 {
        FIB_ZERO
    } else if n == 1 {
        FIB_ONE
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    for i in 1..41 {
       println!("{}: {}", i, fib(i))
    }
}
// fn is_zero(n: i64) -> bool {n == 0}
