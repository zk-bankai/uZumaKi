# Adjusted code to read SHA256 verification data from a JSON file and visualize it

import pandas as pd
import matplotlib.pyplot as plt
import json

def parse_json_file_for_sha256_verification(file_path):
    with open(file_path, "r") as file:
        data = json.load(file)
    
    parsed_data = []
    # Assuming verification data might be structured similarly to proving data
    for category in data["results"]:
        if category["name"] == "SHA256":  # Assuming the same category name for simplicity
            for result in category["results"]:
                parsed_data.append({
                    "Input Size": result["name"],
                    "Cycles": result["metrics"]["cycles"],
                    # Assuming memory usage might not be relevant for verification, hence omitted
                    "Proof Size (bytes)": result["metrics"]["proof_size_bytes"],
                    "Time (seconds)": result["time"]["secs"] + result["time"]["nanos"] / 1e9,
                })
    return parsed_data

def plot_sha256_verification_metrics(df, save_path):
    fig, axs = plt.subplots(1, 2, figsize=(12, 6))  # Creating 1x2 subplots for SHA256 verification only

    axs[0].set_title('Cycles vs Input Size (SHA256 Verification)')
    axs[0].set_xlabel('Input Size')
    axs[0].set_ylabel('Cycles')
    axs[1].set_title('Proof Size vs Input Size (SHA256 Verification)')
    axs[1].set_xlabel('Input Size')
    axs[1].set_ylabel('Proof Size (bytes)')

    axs[0].bar(df["Input Size"], df["Cycles"], color='skyblue')
    axs[1].bar(df["Input Size"], df["Proof Size (bytes)"], color='salmon')

    plt.tight_layout()
    plt.savefig(save_path, format='png', dpi=300)

file_path = "../benchmarks/miden/sha256/verify_bench.json"

# Read and parse the JSON data from the file
df_sha256_verification = pd.DataFrame(parse_json_file_for_sha256_verification(file_path))

save_path_sha256_verification = "../benchmarks/graphs/miden_sha256_verification_metrics.png"

# Generate and save the plots for SHA256 verification data
plot_sha256_verification_metrics(df_sha256_verification, save_path_sha256_verification)

# Note: Replace "path_to_your_sha256_verification_json_file.json" with the actual path to your JSON file before running the script.

