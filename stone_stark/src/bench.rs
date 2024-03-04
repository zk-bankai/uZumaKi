use std::path::Path;

use crate::error::{ProverError, VerifierError};

pub fn run_prover_from_command_line(
    public_input_file: &Path,
    private_input_file: &Path,
    prover_config_file: &Path,
    prover_parameter_file: &Path,
    output_file: &Path,
    benchmark_file: &Path,
    num_of_runs: u32,
) -> Result<(), ProverError> {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!(
            "hyperfine --min-runs {} --export-json {} \"cpu_air_prover \
            --out_file={} \
            --private_input_file={} \
            --public_input_file={} \
            --prover_config_file={} \
            --parameter_file={}\"",
            num_of_runs,
            benchmark_file.display(),
            output_file.display(),
            private_input_file.display(),
            public_input_file.display(),
            prover_config_file.display(),
            prover_parameter_file.display(),
        ))
        .output()?;

    if !output.status.success() {
        return Err(ProverError::CommandError(output));
    }

    Ok(())
}

pub fn run_verifier_from_command_line(
    in_file: &Path,
    benchmark_file: &Path,
    num_of_runs: u32,
) -> Result<(), VerifierError> {

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!(
            "hyperfine --min-runs {} --export-json {} \"cpu_air_verifier \
            --in_file={}\"",
            num_of_runs,
            benchmark_file.display(),
            in_file.display(),
        ))
        .output()?;

    if !output.status.success() {
        return Err(VerifierError::CommandError(output));
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use rstest::rstest;
    use super::*;

    #[rstest]
    fn sha256_8bytes_test_run_prover_verifier_from_command_line() {
        run_prover_from_command_line(
            Path::new("./data/sha256/8bytes/public_input.json"),
            Path::new("./data/sha256/8bytes/private_input.json"),
            Path::new("./data/sha256/8bytes/cpu_air_prover_config.json"),
            Path::new("./data/sha256/8bytes/cpu_air_params.json"),
            Path::new("./data/sha256/8bytes/proof.json"),
            Path::new("../data/stone/sha256/8bytes/prover.json"),
            1
        )
        .unwrap();

        // let proof = read_proof_file(output_file.path());
        // assert_eq!(proof.proof_hex, prover_cli_test_case.proof.proof_hex);

        run_verifier_from_command_line(
            Path::new("./data/sha256/8bytes/proof.json"), 
            Path::new("../data/stone/sha256/8bytes/verifier.json"),
            1
        )
        .expect("Proof file is valid");
    }

    #[rstest]
    fn fibonacci_1_test_run_prover_verifier_from_command_line() {
        run_prover_from_command_line(
            Path::new("./data/fibonacci/fib1/fib1_public_input.json"),
            Path::new("./data/fibonacci/fib1/fib1_private_input.json"),
            Path::new("./data/fibonacci/fib1/cpu_air_prover_config.json"),
            Path::new("./data/fibonacci/fib1/cpu_air_params.json"),
            Path::new("./data/fibonacci/fib1/fib1_proof.json"),
            Path::new("../data/stone/fibonacci/fib1-prover.json"),
            1
        )
        .unwrap();

        // let proof = read_proof_file(output_file.path());
        // assert_eq!(proof.proof_hex, prover_cli_test_case.proof.proof_hex);

        run_verifier_from_command_line(
            Path::new("./data/fibonacci/fib1/fib1-proof.json"), 
            Path::new("../data/stone/fibonacci/fib1-verifier.json"),
            1
        )
        .expect("Proof file is valid");
    }

    #[rstest]
    fn fibonacci_10_test_run_prover_verifier_from_command_line() {
        run_prover_from_command_line(
            Path::new("./data/fibonacci/fib10/fib10_public_input.json"),
            Path::new("./data/fibonacci/fib10/fib10_private_input.json"),
            Path::new("./data/fibonacci/fib10/cpu_air_prover_config.json"),
            Path::new("./data/fibonacci/fib10/cpu_air_params.json"),
            Path::new("./data/fibonacci/fib10/fib10_proof.json"),
            Path::new("../data/stone/fibonacci/fib10-prover.json"),
            1
        )
        .unwrap();

        // let proof = read_proof_file(output_file.path());
        // assert_eq!(proof.proof_hex, prover_cli_test_case.proof.proof_hex);

        run_verifier_from_command_line(
            Path::new("./data/fibonacci/fib10/fib10-proof.json"), 
            Path::new("../data/stone/fibonacci/fib10-verifier.json"),
            1
        )
        .expect("Proof file is valid");
    }

    #[rstest]
    fn fibonacci_100_test_run_prover_verifier_from_command_line() {
        run_prover_from_command_line(
            Path::new("./data/fibonacci/fib100/fib100_public_input.json"),
            Path::new("./data/fibonacci/fib100/fib100_private_input.json"),
            Path::new("./data/fibonacci/fib100/cpu_air_prover_config.json"),
            Path::new("./data/fibonacci/fib100/cpu_air_params.json"),
            Path::new("./data/fibonacci/fib100/fib100_proof.json"),
            Path::new("../data/stone/fibonacci/fib100-prover.json"),
            1
        )
        .unwrap();

        // let proof = read_proof_file(output_file.path());
        // assert_eq!(proof.proof_hex, prover_cli_test_case.proof.proof_hex);

        run_verifier_from_command_line(
            Path::new("./data/fibonacci/fib100/fib100-proof.json"), 
            Path::new("../data/stone/fibonacci/fib100-verifier.json"),
            1
        )
        .expect("Proof file is valid");
    }

    #[rstest]
    fn fibonacci_1000_test_run_prover_verifier_from_command_line() {
        run_prover_from_command_line(
            Path::new("./data/fibonacci/fib1000/fib1000_public_input.json"),
            Path::new("./data/fibonacci/fib1000/fib1000_private_input.json"),
            Path::new("./data/fibonacci/fib1000/cpu_air_prover_config.json"),
            Path::new("./data/fibonacci/fib1000/cpu_air_params.json"),
            Path::new("./data/fibonacci/fib1000/fib1000_proof.json"),
            Path::new("../data/stone/fibonacci/fib1000-prover.json"),
            1
        )
        .unwrap();

        // let proof = read_proof_file(output_file.path());
        // assert_eq!(proof.proof_hex, prover_cli_test_case.proof.proof_hex);

        run_verifier_from_command_line(
            Path::new("./data/fibonacci/fib1000/fib1000-proof.json"), 
            Path::new("../data/stone/fibonacci/fib1000-verifier.json"),
            1
        )
        .expect("Proof file is valid");
    }

    #[rstest]
    fn fibonacci_10_test_run_prover_verifier_from_command_line() {
        run_prover_from_command_line(
            Path::new("./data/fibonacci/fib10000/fib10000_public_input.json"),
            Path::new("./data/fibonacci/fib10000/fib10000_private_input.json"),
            Path::new("./data/fibonacci/fib10000/cpu_air_prover_config.json"),
            Path::new("./data/fibonacci/fib10000/cpu_air_params.json"),
            Path::new("./data/fibonacci/fib10000/fib10000_proof.json"),
            Path::new("../data/stone/fibonacci/fib10000-prover.json"),
            1
        )
        .unwrap();

        // let proof = read_proof_file(output_file.path());
        // assert_eq!(proof.proof_hex, prover_cli_test_case.proof.proof_hex);

        run_verifier_from_command_line(
            Path::new("./data/fibonacci/fib10000/fib10000-proof.json"), 
            Path::new("../data/stone/fibonacci/fib10000-verifier.json"),
            1
        )
        .expect("Proof file is valid");
    }
}
