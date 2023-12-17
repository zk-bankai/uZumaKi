extern crate host;

use benchy::{benchmark, BenchmarkRun};
use host::fib::{provably_fib, verify_fib};


#[benchmark("Fibonacci", [
    ("1", 1),
    // ("10", 10),
    // ("100", 100),
    // ("1000", 1000),
    // ("10000", 10000),
    // ("100000", 100000),
])]
fn fibonacci(b: &mut BenchmarkRun, p: u32) {
    let res = provably_fib(&p);
    b.run(res);
}

#[benchmark("Fibonacci_Verification", [
    ("1", 1),
    // ("10", 10),
    // ("100", 100),
    // ("1000", 1000),
    // ("10000", 10000),
    // ("100000", 100000),
])]
fn fibonacci_ver(b: &mut BenchmarkRun, p: u32) {
    let mut res = provably_fib(&p);

    let (_journal, receipt) = res();

    b.run(verify_fib(receipt));
}

benchy::main!(
    "risczero",
    fibonacci,
    fibonacci_ver
);