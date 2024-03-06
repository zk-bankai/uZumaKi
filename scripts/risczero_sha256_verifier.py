import pandas as pd
import matplotlib.pyplot as plt
import json

def parse_json_file_for_sha256_verification(file_path):
    with open(file_path, "r") as file:
        data = json.load(file)
    
    parsed_data = []
    for category in data["results"]:
        if category["name"] == "Sha256":  # Adjust based on the actual category name for verification
            for result in category["results"]:
                parsed_data.append({
                    "Input Size": result["name"],
                    # Assuming memory usage might not be relevant for this specific data set
                    "Time (seconds)": result["time"]["secs"] + result["time"]["nanos"] / 1e9,
                })
    return parsed_data

def plot_sha256_verification_time(df, save_path):
    plt.figure(figsize=(10, 6))
    plt.title('Sha256 Verification Time for Different Input Sizes')
    plt.xlabel('Input Size')
    plt.ylabel('Time (seconds)')
    plt.plot(df["Input Size"], df["Time (seconds)"], marker='o', linestyle='-', color='blue', label='Verification Time')
    plt.legend()
    plt.tight_layout()
    plt.savefig(save_path, format='png', dpi=300)

# Specify the file path to your JSON data for Sha256 verification
file_path = "../benchmarks/risczero/sha256/verify_bench.json"

# Read and parse the JSON data from the file
df_sha256_verification = pd.DataFrame(parse_json_file_for_sha256_verification(file_path))

save_path = "../benchmarks/graphs/risczero_sha256_verifier_metrics.png"

# Generate and save the plot for Sha256 verification time data
plot_sha256_verification_time(df_sha256_verification, save_path)

# Note: Make sure to replace "path_to_your_sha256_verification_json_file.json" with the actual file path.

