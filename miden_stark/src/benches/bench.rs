extern crate miden_stark;

use benchy::{benchmark, BenchmarkRun};
use miden_stark::fib::fib;

#[benchmark("Fibonacci", [
    ("1", 1),
    ("10", 10),
    ("100", 100),
    ("1000", 1000),
    ("10000", 10000),
    ("100000", 100000),
])]
fn fibonacci(b: &mut BenchmarkRun, p: u32) {
    let (setup, vm) = fib(p);
    let last_vm_state = vm.last().unwrap().unwrap();
    let proof = b.run(setup);
    let proof = proof.to_bytes();
    b.log("proof_size_bytes", proof.len());
    b.log("cycles", last_vm_state.clk as usize);
}

benchy::main!(
    "miden",
    fibonacci,
);