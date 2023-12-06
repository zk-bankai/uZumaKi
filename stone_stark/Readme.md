# Stone Benchmark

### Setup

```sh
cd zk-benchmarks/stone-prover/
docker build --tag stone-prover .
docker run -it stone-prover
```

### Commands inside docker env

```sh

# To build cairo 0 program
# ========================
cairo-compile fibonacci.cairo --output fibonacci_compiled.json --proof_mode

# To Generate Prover Input files
# ==============================

cairo-run \
    --program=fibonacci_compiled.json \
    --layout=small \
    --program_input=fibonacci_input.json \
    --air_public_input=fibonacci_public_input.json \
    --air_private_input=fibonacci_private_input.json \
    --trace_file=fibonacci_trace.json \
    --memory_file=fibonacci_memory.json \
    --print_output \
    --proof_mode

# To Prove
# ========

cpu_air_prover  --out_file=fibonacci_proof.json  --private_input_file=fibonacci_private_input.json     --public_input_file=fibonacci_public_input.json  --prover_config_file=cpu_air_prover_config.json     --parameter_file=cpu_air_params.json  --log_dir=./logs/ --logbuflevel=2

# To verify
# =========

cpu_air_verifier --in_file=fibonacci_proof.json --log_dir=./logs/ --logbuflevel=2
```
