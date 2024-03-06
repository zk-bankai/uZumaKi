import pandas as pd
import matplotlib.pyplot as plt
import json

file_path = "../benchmarks/miden/fibonacci/bench.json"


# Function to parse JSON data into a list of dictionaries for DataFrame creation
def parse_json_file(file_path):
    with open(file_path, "r") as file:
        data = json.load(file)
    
    parsed_data = []
    for category in data["results"]:
        if category["name"] == "Fibonacci_Proof":  # Filter to only include Fibonacci_Proof
            category_name = category["name"]
            for result in category["results"]:
                parsed_data.append({
                    "Category": category_name,
                    "Fibonacci": int(result["name"]),
                    "Cycles": result["metrics"]["cycles"],
                    "Memory Usage (bytes)": result["metrics"].get("memory_usage_bytes", 0),
                    "Proof Size (bytes)": result["metrics"]["proof_size_bytes"],
                    "Time (seconds)": result["time"]["secs"] + result["time"]["nanos"] / 1e9,
                })
    return parsed_data

def plot_all_metrics_in_one(df, save_path):
    fig, axs = plt.subplots(2, 2, figsize=(16, 10))  # Creating 2x2 subplots

    # Subplot 1: Cycles vs Fibonacci
    axs[0, 0].set_title('Cycles vs Fibonacci')
    axs[0, 0].set_xlabel('Fibonacci')
    axs[0, 0].set_xscale('log')
    axs[0, 0].set_ylabel('Cycles')

    # Subplot 2: Memory Usage vs Fibonacci
    axs[0, 1].set_title('Memory Usage vs Fibonacci')
    axs[0, 1].set_xscale('log')
    axs[0, 1].set_xlabel('Fibonacci')
    axs[0, 1].set_ylabel('Memory Usage (bytes)')

    # Subplot 3: Proof Size vs Fibonacci
    axs[1, 0].set_title('Proof Size vs Fibonacci')
    axs[1, 0].set_xscale('log')
    axs[1, 0].set_xlabel('Fibonacci')
    axs[1, 0].set_ylabel('Proof Size (bytes)')

    # Subplot 4: Time vs Fibonacci
    axs[1, 1].set_title('Time vs Fibonacci')
    axs[1, 1].set_xscale('log')
    axs[1, 1].set_xlabel('Fibonacci')
    axs[1, 1].set_ylabel('Time (seconds)')

    subset = df[df["Category"] == "Fibonacci_Proof"]
    axs[0, 0].plot(subset["Fibonacci"], subset["Cycles"], marker="o", label="Fibonacci_Proof", linestyle='-', linewidth=1)
    axs[0, 1].plot(subset["Fibonacci"], subset["Memory Usage (bytes)"], marker="o", label="Fibonacci_Proof")
    axs[1, 0].plot(subset["Fibonacci"], subset["Proof Size (bytes)"], marker="o", label="Fibonacci_Proof")
    axs[1, 1].plot(subset["Fibonacci"], subset["Time (seconds)"], marker="o", label="Fibonacci_Proof")

    for ax in axs.flat:
        ax.legend()

    plt.tight_layout()
    plt.savefig(save_path, format='png', dpi=300)


# Read and parse the JSON data from the file
df = pd.DataFrame(parse_json_file(file_path))

save_path = "../benchmarks/graphs/miden_fibonacci_prover_metrics.png"

# Generate and save the plots for Fibonacci_Proof data
plot_all_metrics_in_one(df, save_path)

