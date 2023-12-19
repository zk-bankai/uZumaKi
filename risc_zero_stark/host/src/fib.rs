use methods::{FIB_ELF, FIB_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt, Journal};


pub fn provably_fib(input: &u32) -> impl FnMut() -> (Journal, Receipt) {
    let env: ExecutorEnv<'_> = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();
    println!(">>> ENV init for prover execution");
    let elf = FIB_ELF;

    move || {
        let prover = default_prover();
        println!(">>> Prover init (default)");
        
        let receipt: Receipt = prover.prove_elf(env.clone(), elf).unwrap();
        println!(">>> Receipt Generated");
        println!("{:?}", receipt.journal);

        let journal = receipt.journal.clone();
        (journal, receipt)
    }
}

pub fn verify_fib(receipt: Receipt) -> impl FnMut() -> i32 {
    let res = receipt.verify(FIB_ID);
    match res {
        Ok(_) => {
            println!("Proof Verified Successfully");

            move || {
                1
            }
        }
        Err(e) => {
            println!("Erro in proof verification");
            println!("{}", e);
            move || {
                0
            }
        }
    }
}