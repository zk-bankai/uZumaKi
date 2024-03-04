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

    /// Check that the Stone Prover command-line wrapper works.
    #[rstest]
    fn sha256_test_run_prover_verifier_from_command_line() {
        run_prover_from_command_line(
            Path::new("./benchmarks/sha/sha256_test_public_input.json"),
            Path::new("./benchmarks/sha/sha256_test_private_input.json"),
            Path::new("./benchmarks/sha/cpu_air_prover_config.json"),
            Path::new("./benchmarks/sha/cpu_air_params.json"),
            Path::new("./benchmarks/sha/sha256_test_proof.json"),
            Path::new("../benchmarks/stone/sha256/sha256_bench_prover.json"),
            1
        )
        .unwrap();

        // let proof = read_proof_file(output_file.path());
        // assert_eq!(proof.proof_hex, prover_cli_test_case.proof.proof_hex);

        run_verifier_from_command_line(
            Path::new("./benchmarks/sha/sha256_test_proof.json"), 
            Path::new("../benchmarks/stone/sha256/sha256_bench_verifier.json"),
            1
        )
        .expect("Proof file is valid");
    }
}
