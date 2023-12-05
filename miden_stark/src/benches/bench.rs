extern crate miden_stark;

use benchy::{benchmark, BenchmarkRun};
use miden_stark::{fib::fib, merkle};
use shared::{
    hash::{rpo::Rpo, HashFn},
    tree_size_n, Tree,
};


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

#[benchmark("Merkle Tree Merge", [
    ("1 + 1", (tree_size_n(0), tree_size_n(0))),
    ("2^10 + 2^10", (tree_size_n(10), tree_size_n(10))),
    ("2^10 + 2^20", (tree_size_n(10), tree_size_n(20))),
    ("2^20 + 2^20", (tree_size_n(20), tree_size_n(20))),
])]
fn merkle_tree_merge(b: &mut BenchmarkRun, (tree1, tree2): (Tree<Rpo>, Tree<Rpo>)) {
    let (prove, iter) = merkle::merge_trees(&tree1, &tree2);

    let proof = b.run(prove);
    let proof_bytes = proof.to_bytes();
    let proof_bytes_zstd = zstd::encode_all(&*proof_bytes, 21).unwrap();

    b.log("proof_size_bytes", proof_bytes.len());
    b.log("compressed_proof_size_bytes", proof_bytes_zstd.len());
    let last_vm_state = iter.last().unwrap().unwrap();

    b.log("cycles", last_vm_state.clk as usize);
}

#[benchmark("Merkle Membership")]
fn merkle_membership(b: &mut BenchmarkRun) {
    let vec = core::iter::from_fn(|| Some(Rpo::random()))
        .take(10)
        .collect();
    let (prove, iter) = merkle::membership(vec, Rpo::random());

    let proof = b.run(prove);
    let proof_bytes = proof.to_bytes();
    let proof_bytes_zstd = zstd::encode_all(&*proof_bytes, 21).unwrap();

    b.log("proof_size_bytes", proof_bytes.len());
    b.log("compressed_proof_size_bytes", proof_bytes_zstd.len());
    let last_vm_state = iter.last().unwrap().unwrap();

    b.log("cycles", last_vm_state.clk as usize);
}

benchy::main!(
    "miden",
    // fibonacci,
    // merkle_tree_merge
    merkle_membership
);