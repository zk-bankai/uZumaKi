# Adjusted code to read SHA256 proving data from a file and visualize it

import pandas as pd
import matplotlib.pyplot as plt
import json

def parse_json_file_for_sha256(file_path):
    with open(file_path, "r") as file:
        data = json.load(file)
    
    parsed_data = []
    for category in data["results"]:
        if category["name"] == "SHA256":  # Filter to only include SHA256
            for result in category["results"]:
                parsed_data.append({
                    "Input Size": result["name"],
                    "Cycles": result["metrics"]["cycles"],
                    "Memory Usage (bytes)": result["metrics"]["memory_usage_bytes"],
                    "Proof Size (bytes)": result["metrics"]["proof_size_bytes"],
                    "Time (seconds)": result["time"]["secs"] + result["time"]["nanos"] / 1e9,
                })
    return parsed_data

def plot_sha256_proving_metrics(df, save_path):
    fig, axs = plt.subplots(1, 3, figsize=(18, 6))  # Creating 1x3 subplots for SHA256 only

    axs[0].set_title('Cycles vs Input Size (SHA256)')
    axs[0].set_xlabel('Input Size')
    axs[0].set_ylabel('Cycles')
    axs[1].set_title('Memory Usage vs Input Size (SHA256)')
    axs[1].set_xlabel('Input Size')
    axs[1].set_ylabel('Memory Usage (bytes)')
    axs[2].set_title('Proof Size vs Input Size (SHA256)')
    axs[2].set_xlabel('Input Size')
    axs[2].set_ylabel('Proof Size (bytes)')

    axs[0].bar(df["Input Size"], df["Cycles"], color='skyblue')
    axs[1].bar(df["Input Size"], df["Memory Usage (bytes)"], color='lightgreen')
    axs[2].bar(df["Input Size"], df["Proof Size (bytes)"], color='salmon')

    plt.tight_layout()
    plt.savefig(save_path, format='png', dpi=300)

# Specify the file path to your JSON data
file_path = "../benchmarks/miden/sha256/bench.json"

# Read and parse the JSON data from the file
df_sha256 = pd.DataFrame(parse_json_file_for_sha256(file_path))

save_path_sha256 = "../benchmarks/graphs/miden_sha256_proving_metrics.png"

# Generate and save the plots for SHA256 proving data
plot_sha256_proving_metrics(df_sha256, save_path_sha256)

# Note: Make sure to update "path_to_your_sha256_json_file.json" with the actual file path before running the script.

