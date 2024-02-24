use methods::{MERKLE_ELF, MERKLE_ID};
use risc0_zkvm::{ default_prover, serde::to_vec, ExecutorEnv, Receipt};
use merkle_tree::{MerkleTree, PrivateInput};

pub fn provably_merkle_membership(vector : Vec<i32>) -> impl FnMut() -> Receipt {
    let mut items: Vec<String> = vec![];

    for ele in vector.iter() {
        items.push(ele.to_string());
    }


    let to_prove_index: u64 = 3;
    let merkle_tree = MerkleTree::new(items.clone());

    move || {
        let input = PrivateInput {
            root: merkle_tree.root(),
            items : items.clone(),
            index_to_prove : to_prove_index
        };

        let prover = default_prover();
        let priv_input = &to_vec(&input).unwrap();
        
        println!("Providing input to prover : {:?}", priv_input);
        let env = ExecutorEnv::builder().write_slice(priv_input).build().unwrap();
    
        println!("Prover init [merkle] ✅");

        let receipt = prover.prove(env, MERKLE_ELF);


        match receipt {
            Ok(receipt) => {
                receipt
            },
            Err(err) => {
                panic!("Error Occurred : {:?} 🔴", err)
            }
        }
    }
}

pub fn verify_merkle(receipt: Receipt) -> impl FnMut() -> i32 {

    let res = receipt.verify(MERKLE_ID);

    match res {
        Ok(_) => {
            println!("Proof Verified Successfully ✅");

            move || {
                1
            }
        }
        Err(e) => {
            println!("Error in proof verification 🔴");
            println!("{}", e);

            move || {
                0
            }
        }
    }
}

// fn main() {
//     env_logger::init();
    
//     let items: Vec<i32> = vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];

//     println!("main init ✅");

//     let mut res = provably_merkle_membership(items);
//     let receipt = res();

//     verify_merkle(receipt);
// }
