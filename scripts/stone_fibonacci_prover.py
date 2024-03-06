import json
import pandas as pd
import matplotlib.pyplot as plt

# List of file paths for the JSON files to read
file_paths = [
    "../benchmarks/stone/fibonacci/fib1-prover.json",
    "../benchmarks/stone/fibonacci/fib10-prover.json",
    "../benchmarks/stone/fibonacci/fib100-prover.json",
    "../benchmarks/stone/fibonacci/fib1000-prover.json",
    "../benchmarks/stone/fibonacci/fib10000-prover.json",
]

# Function to parse a single JSON file and extract relevant data
def parse_json_file(file_path):
    with open(file_path, "r") as file:
        data = json.load(file)
    # Extracting only the relevant metrics, excluding 'command' and 'exit_codes'
    metrics = data["results"][0]  # Assuming each file contains one result
    return {
        "mean": metrics["mean"],
        "stddev": metrics["stddev"],
        "median": metrics["median"],
        "user": metrics["user"],
        "system": metrics["system"],
        "min": metrics["min"],
        "max": metrics["max"],
        "times": metrics["times"]
    }

# Parse all files and store data in a map
prover_data = {}
for file_path in file_paths:
    file_name = file_path.split('/')[-1]  # Extracting file name as identifier
    prover_data[file_name] = parse_json_file(file_path)

# Generate graph
plt.figure(figsize=(10, 6))
means = [data["mean"] for data in prover_data.values()]
file_names = list(prover_data.keys())
plt.bar(file_names, means, color='skyblue')
plt.xlabel('Prover Files')
plt.ylabel('Mean Execution Time (s)')
plt.title('Mean Execution Times of Different Provers')
plt.xticks(rotation=45)
plt.tight_layout()
plt.savefig("../benchmarks/graphs/stone_fibonacci_prover_metrics.png", format='png', dpi=300)

