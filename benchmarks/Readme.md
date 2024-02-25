# Uzumaki Benchmarks

```sh
System Info :
-------------
CPU : Intel i7 12th gen
RAM : 16 GB
GPU : NVIDIA 4050 6 GB, Intel Graphics Driver 6 GB
```

| Language              | Prover          | Verifier                 | Circuit                | Proving Time                                 | Verifying Time                | Proof Type | ENV            |
| --------------------- | --------------- | ------------------------ | ---------------------- | -------------------------------------------- | ----------------------------- | ---------- | -------------- |
| MASM (Miden Assembly) | Miden (Polygon) | Miden_Verifier (Polygon) | Fibonacci              | low : `0.07` s, high : `13.09` s             | low : `47` ns, high : `60` ns | STARK      | prod-proof-gen |
|                       |                 |                          | merkle tree merging    | low : `95.690374` ms, high : `211.495254` ms | low : `57` ns, high : `62` ns | STARK      | prod-proof-gen |
|                       |                 |                          | merkle tree membership | `79.863758` ms                               | `47` ns                       | STARK      | prod-proof-gen |
|                       |                 |                          | sha256                 | low : `3.01` s, high : `60.29` s             | low : `39` ns, high : `45` ns | STARK      | prod-proof-gen |
| Rust                  | RiscZero VM     | RiscZero Verifier        | Fibonacci              | low: `10` s, high : `199` s                  | low : `44` ns, high : `63` ns | STARK      | prod-proof-gen |
|                       |                 |                          | merkle tree membership | low: `19.72` s, high : `749.03` s            | low : `60` ns, high : `76` ns | STARK      | prod-proof-gen |
|                       |                 |                          | sha256                 | low: `20.4` s, high : `348.62` s             | low : `46` ns, high : `52` ns | STARK      | prod-proof-gen |
| Cairo-0               | Stone           | Stone                    | Fibonacci              | `1.84836` s                                  | 100 ns                        | STARK      | prod-proof-gen |

`IMP` : `prod-proof-gen` are the real benchmarks that are taken in prod env
`local-exec` : Local run of the function with benchmarking tool.

## Benchmark Description

- `fibonacci` : taking n (nth fibonacci) from 1 to 1000
- `merkle tree membership` : taking n (number of nodes in a tree) from 5 to 1000
- `merkle tree merging` : taking n (increasing the number of nodes by 2^n for each tree)
- `sha` : We took the benchmark as per n number of bytes. This benchmark input is very different for different provers as follows :
  - miden : n -> 10 - 500 bytes
  - risczero : n -> 10 k - 1000 k bytes
  - `IMP :` This is due to low system configuration as benchy functions exit when there is no extra memory left for the program.
