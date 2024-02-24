mod bench;

use methods::{FIB_ELF, FIB_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt, Journal};


pub fn provably_fib(input: &u32) -> impl FnMut() -> (Journal, Receipt) {
    let env: ExecutorEnv<'_> = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();
    println!(">>> ENV init for prover execution");
    let elf = FIB_ELF;
    // println!("ELF : {:?}", elf);

    move || {
        let prover = default_prover();
        println!(">>> Prover init (default)");
        
        let receipt = prover.prove_elf(env.clone(), elf).unwrap();
        println!(">>> Receipt Generated");
        println!("{:?}", receipt.journal);

        let d: u64 = receipt.journal.decode().unwrap();
        println!("decoded : {}", d);
        
        let journal = receipt.journal.clone();
        (journal, receipt)
    }
}

pub fn verify_fib(receipt: Receipt) {
    let res = receipt.verify(FIB_ID);
    match res {
        Ok(_) => {
            println!("Proof Verified Successfully");
        }
        Err(e) => {
            println!("Error in proof verification");
            println!("{}", e);
        }
    }
}


fn main() {
    env_logger::init();
    
    let input: u32 = 100;

    // println!("ELF : {:?}, ID : {:?}", FIB_ELF, FIB_ID);

    let mut res = provably_fib(&input);
    let (journal, receipt) = res();

    verify_fib(receipt);
    println!(">>> journal : {:?}", journal);
}
