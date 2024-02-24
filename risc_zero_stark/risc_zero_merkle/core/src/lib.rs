// Code source : https://github.com/thor314/circuit-examples/blob/main/risc0-merkle/core/src/lib.rs

extern crate merkle_light_derive;
extern crate merkle_light;

pub mod merkle;

use serde::{Deserialize, Serialize};
pub use merkle::MerkleTree;

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateInput {
    pub root: merkle::Node,
    pub items: Vec<String>,
    pub index_to_prove: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Journal {
    pub proof: merkle::Proof<String>,
    pub root: merkle::Node,
}