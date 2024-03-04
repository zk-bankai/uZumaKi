cairo-compile sha256_test.cairo --output sha256_test_compiled.json --proof_mode
cairo-run \
    --program=sha256_test_compiled.json \
    --layout=recursive \
    --air_public_input=sha256_test_public_input.json \
    --air_private_input=sha256_test_private_input.json \
    --trace_file=sha256_test_trace.bin \
    --memory_file=sha256_test_memory.bin \
    --print_output \
    --proof_mode
/root/Release/src/starkware/main/cpu/cpu_air_prover \
    --out_file=sha256_test_proof.json \
    --private_input_file=sha256_test_private_input.json \
    --public_input_file=sha256_test_public_input.json \
    --prover_config_file=cpu_air_prover_config.json \
    --parameter_file=cpu_air_params.json