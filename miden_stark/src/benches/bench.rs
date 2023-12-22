extern crate miden_stark;

use benchy::{benchmark, BenchmarkRun};
use miden_processor::{StackInputs, ProgramInfo, Kernel};
use miden_stark::{fib::{fib, fib_verify}, merkle::{self, merkle_verify}};
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
    // ("100000", 100000),
])]
fn fibonacci(b: &mut BenchmarkRun, p: u32) {
    // Outputs :
    // ============== 
    // - output_stack
    // - proof
    // - vm_state
    // - program_hash
    let (proof_outputs, vm_outputs) = fib(p);

    let (_output_stack, proof) = proof_outputs();
    let (vm_state, _program_hash) = vm_outputs;
    let _execution_proof = proof;

    let last_vm_state = vm_state.last().unwrap().unwrap();
    let proof = b.run(proof_outputs);
    let proof_parsed = proof.1.to_bytes();
    b.log("proof_size_bytes", proof_parsed.len());
    b.log("cycles", last_vm_state.clk as usize);

    
    // * ============================================
    // * Verification :
    // * ============================================
    
    // ? Need to uncomment when doing the verification metrics
    // let program_info =  ProgramInfo::new(program_hash, Kernel::default());
    
    // let exec = fib_verify(program_info, StackInputs::default(), output_stack, execution_proof);

    // let sec_level = b.run(exec);

    // b.log("Security Level", sec_level.try_into().unwrap());

}

#[benchmark("Fibonacci_Verification", [
    ("1", 1),
    ("10", 10),
    ("100", 100),
    ("1000", 1000),
    ("10000", 10000),
    // ("100000", 100000),
])]
fn fibonacci_verification(b: &mut BenchmarkRun, p: u32) {
 // Outputs :
    // ============== 
    // - output_stack
    // - proof
    // - vm_state
    // - program_hash
    let (proof_outputs, vm_outputs) = fib(p);

    let (_output_stack, proof) = proof_outputs();
    let (vm_state, _program_hash) = vm_outputs;
    let _execution_proof = proof;

    let last_vm_state = vm_state.last().unwrap().unwrap();
    let proof = b.run(proof_outputs);
    let proof_parsed = proof.1.to_bytes();
    b.log("proof_size_bytes", proof_parsed.len());
    b.log("cycles", last_vm_state.clk as usize);

    
    // * ============================================
    // * Verification :
    // * ============================================
    
    // ? Need to uncomment when doing the verification metrics
    let program_info =  ProgramInfo::new(_program_hash, Kernel::default());
    let exec = fib_verify(program_info, StackInputs::default(), _output_stack, _execution_proof);
    let sec_level = b.run(exec);
    b.log("Security Level", sec_level.try_into().unwrap());
}


#[benchmark("Merkle Tree Merge", [
    ("1 + 1", (tree_size_n(0), tree_size_n(0))),
    ("2^10 + 2^10", (tree_size_n(10), tree_size_n(10))),
    ("2^10 + 2^20", (tree_size_n(10), tree_size_n(20))),
    ("2^20 + 2^20", (tree_size_n(20), tree_size_n(20))),
])]
fn merkle_tree_merge(b: &mut BenchmarkRun, (tree1, tree2): (Tree<Rpo>, Tree<Rpo>)) {
    // Outputs :
    // ============== 
    // - output_stack
    // - proof
    // - vm_state
    // - program_hash
    let (proof_outputs, vm_outputs) = merkle::merge_trees(&tree1, &tree2);

    let (_output_stack, proof) = proof_outputs();
    let (vm_state, _program_hash) = vm_outputs;
    let _execution_proof = proof;

    let proof = b.run(proof_outputs);
    let proof_bytes = proof.1.to_bytes();
    let proof_bytes_zstd = zstd::encode_all(&*proof_bytes, 21).unwrap();

    b.log("proof_size_bytes", proof_bytes.len());
    b.log("compressed_proof_size_bytes", proof_bytes_zstd.len());
    let last_vm_state = vm_state.last().unwrap().unwrap();

    b.log("cycles", last_vm_state.clk as usize);

    // * ============================================
    // * Verification :
    // * ============================================

    // ? Need to uncomment when doing the verification metrics
    // let program_info =  ProgramInfo::new(program_hash, Kernel::default());
    
    // let exec = merkle_verify(program_info, StackInputs::default(), output_stack, execution_proof);

    // let sec_level = b.run(exec);

    // b.log("Security Level", sec_level.try_into().unwrap());

}


#[benchmark("Merkle Membership")]
fn merkle_membership(b: &mut BenchmarkRun) {
    let vec = core::iter::from_fn(|| Some(Rpo::random()))
        .take(10)
        .collect();
    // Outputs :
    // ============== 
    // - output_stack
    // - proof
    // - vm_state
    // - program_hash
    let (proof_outputs, vm_outputs) = merkle::membership(vec, Rpo::random());

    let (_output_stack, proof) = proof_outputs();
    let (vm_state, _program_hash) = vm_outputs;
    let _execution_proof = proof;

    let proof = b.run(proof_outputs);
    let proof_bytes = proof.1.to_bytes();
    let proof_bytes_zstd = zstd::encode_all(&*proof_bytes, 21).unwrap();

    b.log("proof_size_bytes", proof_bytes.len());
    b.log("compressed_proof_size_bytes", proof_bytes_zstd.len());
    let last_vm_state = vm_state.last().unwrap().unwrap();

    b.log("cycles", last_vm_state.clk as usize);

    // * ============================================
    // * Verification :
    // * ============================================

    // ? Need to uncomment when doing the verification metrics
    // let program_info =  ProgramInfo::new(program_hash, Kernel::default());
    
    // let exec = merkle_verify(program_info, StackInputs::default(), output_stack, execution_proof);

    // let sec_level = b.run(exec);

    // b.log("Security Level", sec_level.try_into().unwrap());
}

benchy::main!(
    "miden",
    fibonacci,
    fibonacci_verification
    // merkle_tree_merge,
    // merkle_membership
);