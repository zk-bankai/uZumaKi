#![no_main]
#![no_std]

use core::hint::black_box;
// use nalgebra::Matrix2;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    let iterations: u32 = env::read();
    let res = black_box(fib_n(iterations));
    // Commiting the result
    env::commit(&res);
}

/**
 * func : fib_n
 * To get the nth fibonacci number
 */
#[inline(never)]
#[no_mangle]
pub fn fib_n(n: u32) -> u64 {
    let (mut a , mut b) = (0,1);
    if n <= 1 {
        return n as u64;
    }
    let mut i = 2;
    while i <= n {
        let c = a + b;
        a = b;
        b = c;
        i+=1;
    }

    b
}