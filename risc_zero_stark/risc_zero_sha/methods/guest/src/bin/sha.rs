#![no_main]
#![no_std]

use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256}
};

risc0_zkvm::guest::entry!(main);

fn bench_sha(n: usize) {
    let arr = [123u8; 1000];
    for _ in 0..n {
        Impl::hash_bytes(&arr);
    }
}

pub fn main() {
    let n = env::read();
    bench_sha(n)
}