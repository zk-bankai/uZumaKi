use methods::{SHA_ELF, SHA_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt, Journal};


pub fn provably_sha(n: usize) -> impl FnMut() -> (Journal, Receipt) {
    let elf = SHA_ELF;

    move || {
        let env = ExecutorEnv::builder().write(&n).unwrap().build().unwrap();
        println!(">>> ENV init for prover execution");

        let prover = default_prover();
        println!(">>> Prover init (default)");
        
        let receipt: Receipt = prover.prove(env, elf).unwrap();
        println!(">>> Receipt Generated");
        // println!("{:?}", receipt.journal);

        let journal = receipt.journal.clone();
        (journal, receipt)
    }
}

pub fn verify_sha(receipt: Receipt) -> impl FnMut() -> i32 {
    let res = receipt.verify(SHA_ID);
    match res {
        Ok(_) => {
            println!("Proof Verified Successfully");

            move || {
                1
            }
        }
        Err(e) => {
            println!("Error in proof verification");
            println!("{}", e);
            move || {
                0
            }
        }
    }
}