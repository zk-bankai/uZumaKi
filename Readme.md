# Uzumaki

1. **Variety of Systems**: Numerous ZK proof systems exist, each with unique features and optimizations, necessitating a method for comparison.
2. **Targeted Applications**: Different systems are tailored for specific use cases, making benchmarking essential to identify the best fit for a particular application.
3. **No Universal Solution**: Each system has its own trade-offs in terms of efficiency and capabilities, highlighting the need for a comparative analysis.
4. **Rapid Evolution**: The fast-paced innovation in ZK proofs requires a way to track and evaluate new advancements and their impact.
5. **Computational Efficiency**: Benchmarking provides insights into the proof generation and verification speeds of different systems.
6. **Scalability Insights**: Understanding how systems handle increasing computational complexities is crucial for large-scale applications.
7. **Security Comparison**: Evaluating varying security levels across different systems is vital for ensuring data integrity and privacy.
8. **Resource Demand Analysis**: Assessing CPU and memory usage helps in determining the practical deployment of ZK systems.
9. **Informed Decision-Making**: Empirical data from benchmarking aids developers and researchers in choosing the most suitable system.
10. **Standardization Aid**: Benchmarking supports the standardization process in ZK proofs by establishing performance and best practice benchmarks.

Benchmarking ZK proofs is not just about performance metrics; it's a comprehensive process that evaluates suitability for specific applications, scalability, security, and practicality, thereby guiding the evolving landscape of cryptographic privacy and data integrity solutions.

![Pasted image 20231205133256](https://hackmd.io/_uploads/BklKkw2rp.png)

# Comparison between ZK Circuit Development Frameworks

## High-Level Language/DSL

### STARKs

- **Cairo**
  - Arithmetization: AIR
  - Proving system: Stark
- **Typescript-like (Polylang)**
  - Polybase
- **Rust/C/C++ (Risc Zero)**
- **Boojum**
  - Proving system: Stark, ZKSync
- **Starky**
  - Arithmetization: AIR
  - Proving system: Stark, Polygon Zero

### SNARKs

- **Noir**
  - Proving system: Plonk, Aztec
- **Leo**
  - Aleo
- **Rust - crate Bellman**
  - Arithmetization: R1CS
  - Proving system: Groth16, Zcash
- **Circom**
  - Arithmetization: R1CS
  - Proving system: Plonk/Groth16
- **Zokrates**
  - Arithmetization: R1CS
- **Chiquito (Python, Rust)**
- **Snarky (Haskell)**
  - Consensys/Zerocash

## Low-Level Language

### STARKs

- **MASM (Miden Assembly)**
  - Polygon
- **Risc 5**
  - Risc

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

## Testing Systems Under Heavy Load

- Complexity addition via advanced constraints (hashing algorithms, arrays, booleans, data structures, recursion)
  ![Pasted image 20231204093808](https://hackmd.io/_uploads/rJCLJP3B6.png)

## Type of Platform Specification for Testing

- **Linux Server**: 20 Cores @ 2.3 GHz, 384GB memory
- **Macbook M1 Pro**: 10 Cores @ 3.2Ghz, 16GB memory
- Icicle: (TBD)

## Future Work

- Benchmarking sequencers
- Benchmarking different zkVMs (e.g., Scroll, Polygon zkEVM, Consensys zkEVM, zkSync, Risc Zero, zkWasm)
- Benchmarking IR compiler frameworks (e.g., zkLLVM)

Map of curves/Fields with Frameworks and languages
![Pasted image 20231205101458](https://hackmd.io/_uploads/B1ryJwnB6.png)

Model of UI for circuit benchmarks:
![Pasted image 20231204101856](https://hackmd.io/_uploads/H1ibgD2S6.png)
(https://hackmd.io/_uploads/rkrlJP2B6.png)

Metric can be time, ram and proof

Output graph
![Pasted image 20231204101916](https://hackmd.io/_uploads/SJhM1w2r6.png)

Stats:
![Pasted image 20231204101940](https://hackmd.io/_uploads/B1Z7yPnBp.png)

Arithmetic Backends
![Pasted image 20231204102116](https://hackmd.io/_uploads/B1xAJvnra.png)
Output Graph:
![Pasted image 20231204102136](https://hackmd.io/_uploads/rywE1P2rT.png)

Elliptic curve benchmarks
![Pasted image 20231204102218](https://hackmd.io/_uploads/ByT4Jw2rT.png)

Output graph
![Pasted image 20231204102239](https://hackmd.io/_uploads/BkfSJP3Bp.png)

TBD:
Quantitive costs

- Compilation time
- Prover time
- Prover space
- Verifier time
- Verifier space
- Proof size
- Size of public keys and parameters
- Rounds of interaction
- Security level (statistical or computational)

Qualitative costs

- Hardness assumptions (comp vs. PQ)
- Setup assumptions (SRS, URS; universal / specific; updateable?)
- Zero-knowledge (statistical vs comp.)
- Simplicity and ease of verifying correctness
- Parallelization and acceleration
- Can we parallelize or distribute the prover’s computation?
  Qualitative
  Costs

# Resources

### Github Repositories

- https://github.com/delendum-xyz/zk-benchmarking
- https://github.com/zkCollective/zk-Harness
- https://github.com/celer-network/zk-benchmark
- https://github.com/polybase/zk-benchmarks
- https://github.com/delendum-xyz/zk-benchmarking
- https://github.com/ingonyama-zk/icicle

### Articles

- https://ethresear.ch/t/benchmarking-zkp-development-frameworks-the-pantheon-of-zkp/14943
- https://hackmd.io/@heliax/SJU01u5fs
- https://eprint.iacr.org/2023/1503

### Benchmarking

- https://www.zk-bench.org/
- https://zkbench.dev/
