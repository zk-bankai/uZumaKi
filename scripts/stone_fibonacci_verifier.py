import pandas as pd
import matplotlib.pyplot as plt
import json

def parse_json_for_verifier(file_path):
    with open(file_path, "r") as file:
        data = json.load(file)
    
    # Extract relevant data, ignoring "command" and "exit_codes"
    results = [{
        "mean": item["mean"],
        "stddev": item["stddev"],
        "median": item["median"],
        "min": item["min"],
        "max": item["max"]
    } for item in data["results"]]
    
    return results

file_paths = [
    "../benchmarks/stone/fibonacci/fib1-verifier.json",
    "../benchmarks/stone/fibonacci/fib10-verifier.json",
    "../benchmarks/stone/fibonacci/fib100-verifier.json",
    "../benchmarks/stone/fibonacci/fib1000-verifier.json",
    "../benchmarks/stone/fibonacci/fib10000-verifier.json",
]
data_map = {path: parse_json_for_verifier(path) for path in file_paths}

means = [data[0]["mean"] for data in data_map.values()]  # Extracting the mean time from each dataset
labels = ["Fib1", "Fib2", "Fib3", "Fib4", "Fib5"]  # Custom labels based on file names or other criteria

plt.figure(figsize=(10, 6))
plt.plot(labels, means, marker='o', linestyle='-', color='blue', label='Verification Time')
plt.title('Fibonacci Verifier Performance')
plt.xlabel('Test Case')
plt.ylabel('Mean Time (seconds)')
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.savefig("../benchmarks/graphs/stone_fibonacci_verifier_metrics.png", format='png', dpi=300)


