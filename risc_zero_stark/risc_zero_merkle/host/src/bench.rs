extern crate host;

use benchy::{benchmark, BenchmarkRun};
use host::merkle::{provably_merkle_membership, verify_merkle};

#[benchmark("MerkleTreeMembership", [
    ("Number of tree nodes : 5", generate_array(5)),
    ("Number of tree nodes : 10", generate_array(10)),
    ("Number of tree nodes : 100", generate_array(100)),
    ("Number of tree nodes : 1000", generate_array(1000)),
    // ("Number of tree nodes : 10000", generate_array(10000)),
])]
fn merkle_tree_membership(b : &mut BenchmarkRun, items: Vec<i32>){
    let res = provably_merkle_membership(items);

    b.run(res);
}

#[benchmark("MerkleTreeMembershipVerification", [
    ("Number of tree nodes : 5", generate_array(5)),
    ("Number of tree nodes : 10", generate_array(10)),
    ("Number of tree nodes : 100", generate_array(100)),
    ("Number of tree nodes : 1000", generate_array(1000)),
    // ("Number of tree nodes : 10000", generate_array(10000)),
])]
fn merkle_tree_membership_verify(b : &mut BenchmarkRun, items: Vec<i32>){
    let mut res = provably_merkle_membership(items);

    let receipt = res();

    b.run(verify_merkle(receipt));
}

fn generate_array(size: i32) -> Vec<i32> {
    let mut items : Vec<i32> = vec![];

    for i in 1..size {
        items.push(i);
    }

    items
}

benchy::main!(
    "risczero",
    // merkle_tree_membership,
    merkle_tree_membership_verify
);