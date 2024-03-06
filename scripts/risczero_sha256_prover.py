import pandas as pd
import matplotlib.pyplot as plt
import json

def parse_json_file_for_sha256(file_path):
    with open(file_path, "r") as file:
        data = json.load(file)
    
    parsed_data = []
    for category in data["results"]:
        if category["name"] == "Sha256":  # Focus only on Sha256 data
            for result in category["results"]:
                parsed_data.append({
                    "Input Size": result["name"],
                    "Memory Usage (bytes)": result["metrics"]["memory_usage_bytes"],
                    "Time (seconds)": result["time"]["secs"] + result["time"]["nanos"] / 1e9,
                })
    return parsed_data

def plot_sha256_metrics(df, save_path):
    fig, ax1 = plt.subplots(figsize=(10, 6))

    ax1.set_title('Sha256 Performance Metrics')
    ax1.set_xlabel('Input Size')
    ax1.set_ylabel('Memory Usage (bytes)', color='tab:red')
    ax1.plot(df["Input Size"], df["Memory Usage (bytes)"], color='tab:red', marker='o', linestyle='-', label='Memory Usage')
    ax1.tick_params(axis='y', labelcolor='tab:red')

    ax2 = ax1.twinx()  # instantiate a second axes that shares the same x-axis
    ax2.set_ylabel('Time (seconds)', color='tab:blue')  # we already handled the x-label with ax1
    ax2.plot(df["Input Size"], df["Time (seconds)"], color='tab:blue', marker='x', linestyle='-', label='Time')
    ax2.tick_params(axis='y', labelcolor='tab:blue')

    fig.tight_layout()  # otherwise the right y-label is slightly clipped
    plt.savefig(save_path, format='png', dpi=300)

# Specify the file path to your JSON data for Sha256
file_path_sha256 = "../benchmarks/risczero/sha256/bench.json"

# Read and parse the JSON data from the file
df_sha256 = pd.DataFrame(parse_json_file_for_sha256(file_path_sha256))

save_path_sha256 = "../benchmarks/graphs/risczero_sha256_prover_metrics.png"

# Generate and save the plots for Sha256 data
plot_sha256_metrics(df_sha256, save_path_sha256)

# Note: Make sure to update "path_to_your_sha256_json_file.json" with the actual path to your JSON file before running the script.

