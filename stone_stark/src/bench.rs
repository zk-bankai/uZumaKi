use std::path::Path;
use std::path::PathBuf;
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

fn run_test_case(
    test_category: &str,
    test_name: &str,
    prover_json: &str,
    verifier_json: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = PathBuf::from(format!("./data/{}/{}", test_category, test_name));
    let benchmarks_path = PathBuf::from("../benchmarks/stone");

    run_prover_from_command_line(
        &base_path.join("public_input.json"),
        &base_path.join("private_input.json"),
        &base_path.join("cpu_air_prover_config.json"),
        &base_path.join("cpu_air_params.json"),
        &base_path.join("proof.json"),
        &benchmarks_path.join(prover_json),
        1,
    )?;

    run_verifier_from_command_line(
        &base_path.join("proof.json"),
        &benchmarks_path.join(verifier_json),
        1,
    )?;

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn sha256_8bytes_test() -> Result<(), Box<dyn std::error::Error>> {
        run_test_case("sha256", "8bytes", "sha256/8B-prover.json", "sha256/8B-verifier.json")
    }
    
    #[rstest]
    fn sha256_100bytes_test() -> Result<(), Box<dyn std::error::Error>> {
        run_test_case("sha256", "100bytes", "sha256/100B-prover.json", "sha256/100B-verifier.json")
    }

    #[rstest]
    fn fibonacci_1_test() -> Result<(), Box<dyn std::error::Error>> {
        run_test_case("fibonacci", "fib1", "fibonacci/fib1-prover.json", "fibonacci/fib1-verifier.json")
    }

    #[rstest]
    fn fibonacci_10_test() -> Result<(), Box<dyn std::error::Error>> {
        run_test_case("fibonacci", "fib10", "fibonacci/fib10-prover.json", "fibonacci/fib10-verifier.json")
    }

    #[rstest]
    fn fibonacci_100_test() -> Result<(), Box<dyn std::error::Error>> {
        run_test_case("fibonacci", "fib100", "fibonacci/fib100-prover.json", "fibonacci/fib100-verifier.json")
    }

    #[rstest]
    fn fibonacci_1000_test() -> Result<(), Box<dyn std::error::Error>> {
        run_test_case("fibonacci", "fib1000", "fibonacci/fib1000-prover.json", "fibonacci/fib1000-verifier.json")
    }

    #[rstest]
    fn fibonacci_10000_test() -> Result<(), Box<dyn std::error::Error>> {
        run_test_case("fibonacci", "fib10000", "fibonacci/fib10000-prover.json", "fibonacci/fib10000-verifier.json")
    }
}

