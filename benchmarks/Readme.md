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
|                       |                 |                          | merkle tree merging    | low : `95.690374` ms, high : `211.495254` ms | low : `57` ns, high : `62` ns | STARK      | local-exec     |
|                       |                 |                          | merkle tree membership | `79.863758` ms                               | `47` ns                       | STARK      | local-exec     |
| Cairo-0               | Stone           | Stone                    | Fibonacci              | `1.84836` s                                  | 100 ns                        | STARK      | local-exec     |
| Rust                  | RiscZero VM     | RiscZero Verifier        | Fibonacci              | low: `10` s, high : `199` s                  | low : `44` ns, high : `63` ns | STARK      | prod-proof-gen |

`IMP` : `prod-proof-gen` are the real benchmarks that are taken in prod env
`local-exec` : Local run of the function with benchmarking tool.
