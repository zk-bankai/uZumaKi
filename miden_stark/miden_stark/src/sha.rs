use miden::{AdviceInputs, Assembler, Digest, ProgramInfo, ProofOptions, StackOutputs};
use miden_processor::{MemAdviceProvider, StackInputs, VmStateIterator};
use miden_prover::ExecutionProof;

pub fn sha(n_bytes: usize) -> (impl Fn() -> (StackOutputs, ExecutionProof), (VmStateIterator, Digest)) {

    let sha_ops = f64::ceil(n_bytes as f64 / 4. /8.);
    let code = format!(
        r#"
        use.std::crypto::hashes::sha256
        begin
            repeat.{sha_ops}
                push.9.10.11.12.13.14.15.16
                exec.sha256::hash_2to1
            end
        end
        "#
    );


    let assembler = Assembler::default().with_library(&miden_stdlib::StdLibrary::default()).unwrap();

    let program = assembler.compile(code.clone());

    match program {
        Ok(_) => {},
        Err(e) => {
            println!("Compilation Error : {:?}", e);
        }
    }

    let program = assembler.compile(code).unwrap();

    let program_hash = program.hash().clone();
    let advice_provider = MemAdviceProvider::from(
        AdviceInputs::default()
            .with_stack_values(vec![n_bytes as u64])
            .unwrap(),
    );

    let vm: miden_processor::VmStateIterator =
        miden_processor::execute_iter(&program, StackInputs::default(), advice_provider.clone());

    (
        move || {
            let (_stack, _proof) = miden::prove(&program, StackInputs::default(), MemAdviceProvider::default(), ProofOptions::default()).unwrap();

            (_stack, _proof)
        },
        (vm, program_hash)
    )
}

pub fn sha_verify(program_info : ProgramInfo, stack_inputs : StackInputs, stack_outputs: StackOutputs, execution_proof : ExecutionProof) -> impl Fn() -> u32 {

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