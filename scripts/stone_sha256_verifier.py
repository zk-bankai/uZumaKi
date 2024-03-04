import pandas as pd
import matplotlib.pyplot as plt
import json
import numpy as np

def parse_json_for_sha256_verifier(file_path):
    with open(file_path, "r") as file:
        data = json.load(file)
    mean_times = [result["mean"] for result in data["results"]]
    return np.mean(mean_times)

file_paths = [
    "../benchmarks/stone/sha256/8B-verifier.json",
    "../benchmarks/stone/sha256/100B-verifier.json",
]
mean_times_verifier = [parse_json_for_sha256_verifier(file) for file in file_paths]
input_sizes_verifier = ["8B", "100B"]

plt.figure(figsize=(10, 6))
plt.plot(input_sizes_verifier, mean_times_verifier, marker='o', linestyle='-', color='blue', label='SHA256 Verifier Mean Time')
plt.title('SHA256 Verifier Mean Times for Different Input Sizes')
plt.xlabel('Input Size')
plt.ylabel('Mean Time (seconds)')
plt.grid(True)
plt.savefig("../benchmarks/graphs/stone_sha256_verifier_metrics.png", format='png', dpi=300)
