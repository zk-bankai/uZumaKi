pub mod merkle;

#[cfg(test)]
mod merkle_tests {
    use crate::merkle::{provably_merkle_membership, verify_merkle};

    #[test]
    fn test_prover() {
        let vector: Vec<i32> = vec![1,2,3,4,5];
        let mut res = provably_merkle_membership(vector);
        let receipt = res();
        assert!(receipt.journal.bytes.len() > 1);
    }

    #[test]
    fn test_verifier() {
        let vector: Vec<i32> = vec![1,2,3,4,5];
        let mut res = provably_merkle_membership(vector);
        let receipt = res();
        assert!(receipt.journal.bytes.len() > 1);

        let mut verify = verify_merkle(receipt);

        let res_verify  = verify();
        assert_eq!(res_verify, 1);
    }
}