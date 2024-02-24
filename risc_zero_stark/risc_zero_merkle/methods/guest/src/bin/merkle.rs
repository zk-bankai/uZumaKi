#![no_main]
// #![no_std]

use risc0_zkvm::guest::env;
use merkle_tree::{Journal, MerkleTree, PrivateInput};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let input: PrivateInput = env::read();
    let item_to_prove = input.items[input.index_to_prove as usize].clone();

    // creating the merkle tree with input elements
    let merkle_tree = MerkleTree::new(input.items);

    // generating proof with public input as index
    let proof = merkle_tree.prove(input.index_to_prove as usize);

    assert!(proof.verify(&input.root, &item_to_prove));

    let journal = Journal {
        root: input.root,
        proof,
    };

    env::commit(&journal);
}
