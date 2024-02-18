use miden::Assembler;
use miden_crypto::merkle::InnerNodeInfo;
use miden_processor::{AdviceInputs, MemAdviceProvider, StackInputs, VmStateIterator, ProgramInfo, StackOutputs};
use miden_prover::{ExecutionProof, ProofOptions};
use miden_crypto::hash::rpo::RpoDigest;
use shared::{
    hash::{rpo::Rpo, HashFn},
    Tree,
};


pub fn membership(
    path: Vec<<Rpo as HashFn>::Digest>,
    digest: <Rpo as HashFn>::Digest,
) -> (impl Fn() -> (StackOutputs, ExecutionProof), (VmStateIterator, RpoDigest)) {
    assert_eq!(path.len(), 10);
    let mut advice = AdviceInputs::default();
    advice.extend_stack(path.iter().flat_map(|digest| digest.as_elements()).copied());
    advice.extend_stack(digest.as_elements().iter().copied());
    let advice = MemAdviceProvider::from(advice);

    let program = Assembler::default()
        .compile(include_str!("./asm/Membership/membership.masm"))
        .unwrap();

    let program_hash = program.hash().clone();

    let stack = StackInputs::default();
    let opts = ProofOptions::default();

    let vm_iter = miden_processor::execute_iter(&program, stack.clone(), advice.clone());

    (
        move || {
            let (_stack,proof) = miden_prover::prove(&program, stack.clone(), advice.clone(), opts.clone()).unwrap();

            (_stack, proof)
        },
        (vm_iter, program_hash)
    )

}

pub fn merge_trees(
    tree1: &Tree<Rpo>,
    tree2: &Tree<Rpo>,
) -> (impl Fn() -> (StackOutputs, ExecutionProof), (VmStateIterator, RpoDigest)) {
    let mut advice = AdviceInputs::default();
    advice.extend_merkle_store(tree1.iter().map(inner_node_info));
    advice.extend_merkle_store(tree2.iter().map(inner_node_info));

    let advice = MemAdviceProvider::from(advice);
    let program = Assembler::default()
        .compile("begin mtree_merge end")
        .unwrap();
    let program_hash = program.hash().clone();
    let stack = [tree1, tree2]
        .into_iter()
        .flat_map(|tree| tree.digest().as_elements().to_owned())
        .collect();

    let stack = StackInputs::new(stack);
    let opts = ProofOptions::default();

    let vm_iter = miden_processor::execute_iter(&program, stack.clone(), advice.clone());

    (
        move || {
            let (_stack,proof) = miden_prover::prove(&program, stack.clone(), advice.clone(), opts.clone()).unwrap();

            (_stack, proof)
        },
        (vm_iter, program_hash)
    )
}

fn inner_node_info(tree: &Tree<Rpo>) -> InnerNodeInfo {
    match tree {
        Tree::Leaf(digest) => InnerNodeInfo {
            value: *digest,
            left: Rpo::null(),
            right: Rpo::null(),
        },
        Tree::Node {
            left,
            right,
            digest,
        } => InnerNodeInfo {
            value: *digest,
            left: left.digest(),
            right: right.digest(),
        },
    }
}

pub fn merkle_verify(program_info : ProgramInfo, stack_inputs : StackInputs, stack_outputs: StackOutputs, execution_proof : ExecutionProof) -> impl Fn() -> u32 {

    let v_res = miden_verifier::verify(program_info, stack_inputs, stack_outputs, execution_proof);


    let err_code = 0;
    let res;

    match v_res {
        Ok(sec_level) => {
            res = sec_level;
        },
        Err(err) => {
            println!("Error : [merkle_verify] : {}", err);
            res = err_code;
        }
    }

    move || res
}