extern crate host;

use benchy::{benchmark, BenchmarkRun};
use host::sha::{provably_sha, verify_sha};


#[benchmark("Sha256", [
    ("10 bytes", 10),
    ("100 bytes", 100),
    ("500 bytes", 500),
    ("1000 bytes", 1000),
])]
fn fibonacci(b: &mut BenchmarkRun, p: usize) {
    let res = provably_sha(p);
    b.run(res);
}

#[benchmark("Sha256", [
    ("10 bytes", 10),
    ("100 bytes", 100),
    // ("500 bytes", 500),
    // ("1000 bytes", 1000),
])]
fn fibonacci_verify(b: &mut BenchmarkRun, p: usize) {
    let mut res = provably_sha(p);
    let receipt = res();
    b.run(verify_sha(receipt.1));
}

benchy::main!(
    "risczero",
    // fibonacci,
    fibonacci_verify
);