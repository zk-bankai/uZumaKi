# Roadmap

## Index

- [Roadmap](./Roadmap.md#roadmap)
  - [Index](#index)
  - [High Level DSL](#high-level-languagedsl)
    - [STARKs](#starks)
    - [SNARKs](#snarks)
  - [Low Level DSL](#low-level-language)
    - [STARKs](#stark)
  - [Proof System](#proof-systems)
  - [Groth 16 Frameworks](#groth16-frameworks)
  - [Plonk Frameworks](#plonk-frameworks)
  - [Arithmetic Operations](#arithmetic-operations)
  - [EC Operations](#elliptic-curve-operations)
  - [DSL Circuit benchmarks](#dsl-circuit-benchmarks-without-proving-system)
  - [Assessments](#assessments)
  - [Additional Assessments](#additional-assessments)
  - [Testing Systems Under Heavy Load](#testing-systems-under-heavy-load)
  - [Type of Platform Specification for Testing](#type-of-platform-specification-for-testing)
  - [Future Work](#future-work)

## Notation

```sh
🌀 : Demo phase
⭕ : To start
✅ : completed
```

## High-Level Language/DSL

### STARKs

| Prover         | Language/Library           | Arithmetization | Status |
| -------------- | -------------------------- | --------------- | ------ |
| Stone          | Cairo                      | AIR             | 🌀     |
| Miden          | PolyLang (typescript-like) | -               | 🌀     |
| RiskZero zkVM  | Rust, C , C++              | -               | 🌀     |
| Boojum(ZKSync) | Rust , C , C++             | -               | ⭕     |

### SNARKs

| Prover         | Language/Library | Arithmetization | Status |
| -------------- | ---------------- | --------------- | ------ |
| Plonk          | Noir             | -               | ⭕     |
| Aleo           | Leo              | -               | ⭕     |
| Groth16        | Bellman (Rust)   | R1CS            | ⭕     |
| Groth16        | Circom           | R1CS            | ⭕     |
| Marlin/Groth16 | Zokrates         | R1CS            | ⭕     |

## Low-Level Language

### STARK

| Language |
| -------- |
| MASM     |
| Risc 5   |

## Proof Systems

- Groth16
- Plonk
- Marlin/Marlin'
- Stark

## Groth16 Frameworks

- Gnark
- Rapidsnark
- Arkworks
- Snarkjs
- Bellman
- Zokrates
- Libsnark

## Plonk Frameworks

- Plonky2
- Halo2
- Aztec (Implementation of Plonk)
- Hercules (Rust-based with Plonk support)

## Arithmetic Operations

- inv
- mul
- sub
- exp
- add

## Elliptic Curve Operations

- g1-scalar-multiplication
- g2-multi-scalar-multiplication
- pairing
- g2-scalar-multiplication
- g1-multi-scalar-multiplication

## DSL circuit Benchmarks without proving system

- Independent of proving scheme limitations: Some proving systems may have limitations or optimizations that can skew the understanding of a DSL's capabilities. Comparing DSLs independently allows for an evaluation that is not influenced by such factors.
- By comparing DSLs independently of specific proving systems, you can focus on the efficiency and optimization of circuit design. This allows for an assessment of how well each DSL facilitates the creation of efficient and optimized circuits.
- Language features, learning curve
- Analysis under heavy load
- Tooling and ecosystem support

## Assessments

- Prover performance
- Verifier performance
- Proof size
- Proof Generation Time (including witness generation time)
- Peak Memory usage during proof generation
- Average CPU Utilization % during proof generation (Reflects parallelization degree)
- Proof cost (Dependent on field and curve efficiency, proof techniques, and computation model)
- EVM Verifier
- External libraries support
- Ease of Use: Learning curve and user-friendliness of each DSL
- Security Features: Built-in security measures of each DSL
- Community and Ecosystem: Community size, resources, documentation, and support
- Version Tracking: Include version numbers of DSLs for updates and improvements
- Parallelization and Scalability: Support for parallel computations and scaling

## Additional Assessments

### 1. Apple to Apple comparison between different hardware systems

Modular Arithmetic Focus: Prioritize modular multiplication operations per second (MMOPS) as a key metric, offering a more concrete and comparable measure across different systems.
Field-Specific Benchmarks: Include benchmarks for different field sizes (e.g., 256-bit, 384-bit) to capture performance nuances across various cryptographic fields.

### 2. Comprehensive Power Efficiency Analysis

MMOPS/Watt Metric: Adopt a standardized MMOPS/Watt metric for a direct comparison of power efficiency across different hardware setups.
Total Cost of Ownership (TCO): Include a more detailed analysis of TCO, factoring in hardware costs, operational expenses, and potential resale values to provide a holistic view of economic efficiency.

### 3. Hardware and System Diversity

Diverse Hardware Testing: Test ZKP systems on a variety of hardware, including CPUs, GPUs, FPGAs, and ASICs, to understand performance across different computational platforms.
System Scalability Analysis: Assess how systems scale with increased complexity and workload, providing insights into their real-world applicability.

## Testing Systems Under Heavy Load

- Complexity addition via advanced constraints (hashing algorithms, arrays, booleans, data structures, recursion)
  ![Pasted image 20231204093808](https://hackmd.io/_uploads/rJCLJP3B6.png)

## Type of Platform Specification for Testing

- **Linux Server**: 20 Cores @ 2.3 GHz, 384GB memory
- **Macbook M1 Pro**: 10 Cores @ 3.2Ghz, 16GB memory
- Icicle: (TBD)

## Future Work

- DSL frameworks without proving systems
- Compute on Icicle
- Benchmarking sequencers
- Benchmarking different zkVMs (e.g., Scroll, Polygon zkEVM, Consensys zkEVM, zkSync, Risc Zero, zkWasm)
- Benchmarking IR compiler frameworks (e.g., zkLLVM)
