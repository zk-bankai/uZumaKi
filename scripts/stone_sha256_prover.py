import pandas as pd
import matplotlib.pyplot as plt
import json
import numpy as np

def parse_json_for_prover(file_path):
    with open(file_path, "r") as file:
        data = json.load(file)
    mean_times = [result["mean"] for result in data["results"]]
    return np.mean(mean_times)

file_paths = [
    "../benchmarks/stone/sha256/8B-prover.json",
    "../benchmarks/stone/sha256/100B-prover.json",
]
mean_times = [parse_json_for_prover(file) for file in file_paths]
input_sizes = ["8B", "100B"]

plt.figure(figsize=(10, 6))
plt.plot(input_sizes, mean_times, marker='o', linestyle='-', color='blue')
plt.title('SHA256 Prover Mean Times for Different Input Sizes')
plt.xlabel('Input Size')
plt.ylabel('Mean Time (seconds)')
plt.grid(True)
plt.savefig("../benchmarks/graphs/stone_sha256_prover_metrics.png", format='png', dpi=300)

