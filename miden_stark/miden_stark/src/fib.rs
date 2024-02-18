use miden::{Assembler, ProofOptions};
// use miden_crypto::hash::Digest;
use miden_processor::{AdviceInputs, MemAdviceProvider, StackInputs, VmStateIterator, StackOutputs, ProgramInfo};
use miden_prover::ExecutionProof;
use miden::crypto::RpoDigest;


pub fn fib(n: u32) -> (impl Fn() -> (StackOutputs, ExecutionProof), (VmStateIterator, RpoDigest)) {
    let code = format!(
        r#"
        begin
            push.0
            push.0
            push.0
            push.1
            repeat.{n}
                dup
                movup.3
                u32overflowing_add
                dup.3
                movup.4
                u32overflowing_add3
                drop
                swap
            end
        end
    "#
    );
 
    let assembler = Assembler::default()
        .with_library(&miden_stdlib::StdLibrary::default())
        .unwrap();
    let program = assembler.compile(code).unwrap();
    let program_hash = program.hash().clone();
    let advice_provider = MemAdviceProvider::from(
        AdviceInputs::default()
            .with_stack_values(vec![n as u64])
            .unwrap(),
    );
    let vm: miden_processor::VmStateIterator =
        miden_processor::execute_iter(&program, StackInputs::default(), advice_provider.clone());

    (
        move || {
            let (_stack, proof) = miden::prove(
                &program,
                StackInputs::default(),
                advice_provider.clone(),
                ProofOptions::default(),
            )
            .unwrap();

            (_stack,proof)
        },
        (vm,program_hash)
    )
}


pub fn fib_verify(program_info : ProgramInfo, stack_inputs : StackInputs, stack_outputs: StackOutputs, execution_proof : ExecutionProof) -> impl Fn() -> u32 {

    let v_res = miden_verifier::verify(program_info, stack_inputs, stack_outputs, execution_proof);


    let err_code = 0;
    let res;

    match v_res {
        Ok(sec_level) => {
            res = sec_level;
        },
        Err(err) => {
            println!("Error : {}", err);
            res = err_code;
        }
    }

    move || res
}
