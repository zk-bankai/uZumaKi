import pandas as pd
import matplotlib.pyplot as plt
import json

def parse_json_file_for_fibonacci_proving(file_path):
    with open(file_path, "r") as file:
        data = json.load(file)
    
    parsed_data = []
    for category in data["results"]:
        if category["name"] == "Fibonacci_Proving":  # Focus only on Fibonacci Proving data
            for result in category["results"]:
                parsed_data.append({
                    "Fibonacci Number": result["name"],
                    "Memory Usage (bytes)": result["metrics"]["memory_usage_bytes"],
                    "Time (seconds)": result["time"]["secs"] + result["time"]["nanos"] / 1e9,
                })
    return parsed_data

def plot_fibonacci_proving_metrics(df, save_path):
    fig, ax1 = plt.subplots(figsize=(10, 6))

    ax1.set_title('Fibonacci Proving Performance Metrics')
    ax1.set_xlabel('Fibonacci Number')
    ax1.set_ylabel('Memory Usage (bytes)', color='tab:red')
    ax1.plot(df["Fibonacci Number"], df["Memory Usage (bytes)"], color='tab:red', marker='o', label='Memory Usage')
    ax1.tick_params(axis='y', labelcolor='tab:red')

    ax2 = ax1.twinx()  # instantiate a second axes that shares the same x-axis
    ax2.set_ylabel('Time (seconds)', color='tab:blue')  # we already handled the x-label with ax1
    ax2.plot(df["Fibonacci Number"], df["Time (seconds)"], color='tab:blue', marker='x', label='Time')
    ax2.tick_params(axis='y', labelcolor='tab:blue')

    fig.tight_layout()  # otherwise the right y-label is slightly clipped
    plt.savefig(save_path, format='png', dpi=300)

# Specify the file path to your JSON data for Fibonacci Proving
file_path = "../benchmarks/risczero/fibonacci/bench.json"

# Read and parse the JSON data from the file
df_fibonacci_proving = pd.DataFrame(parse_json_file_for_fibonacci_proving(file_path))

save_path_fibonacci_proving = "../benchmarks/graphs/risczero_fibonacci_proving_metrics.png"

# Generate and save the plots for Fibonacci Proving data
plot_fibonacci_proving_metrics(df_fibonacci_proving, save_path_fibonacci_proving)

# Note: Make sure to update "path_to_your_fibonacci_proving_json_file.json" with the actual path to your JSON file before running the script.

