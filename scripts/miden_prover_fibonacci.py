import json
import pandas as pd
import matplotlib.pyplot as plt

# Load the JSON data from the file
file_path = './benchmarks/miden/fibonacci/bench.json'  # Replace with the path to your JSON file

with open(file_path, 'r') as file:
    data = json.load(file)

# Extracting data for visualization
results = data['results'][0]['results']  # Assuming we are focusing on the first entry in 'results'

# Creating a DataFrame
df = pd.DataFrame({
    'Fibonacci': [int(r['name']) for r in results],  # converting names to integers
    'cycles': [r['metrics']['cycles'] for r in results],
    'memory_usage_bytes': [r['metrics']['memory_usage_bytes'] for r in results],
    'proof_size_bytes': [r['metrics']['proof_size_bytes'] for r in results],
    'time_seconds': [r['time']['secs'] + r['time']['nanos'] / 1e9 for r in results]
})

# Creating visualizations
plt.figure(figsize=(16, 10))

# Plotting 'cycles' vs 'Fibonacci'
plt.subplot(2, 2, 1)
plt.plot(df['Fibonacci'], df['cycles'], marker='o')
plt.xlabel('Fibonacci')
plt.ylabel('Cycles')
plt.title('Cycles vs Fibonacci')
plt.xscale('log')

# Plotting 'memory_usage_bytes' vs 'Fibonacci'
plt.subplot(2, 2, 2)
plt.plot(df['Fibonacci'], df['memory_usage_bytes'], marker='o', color='green')
plt.xlabel('Fibonacci')
plt.ylabel('Memory Usage (bytes)')
plt.title('Memory Usage vs Fibonacci')
plt.xscale('log')

# Plotting 'proof_size_bytes' vs 'Fibonacci'
plt.subplot(2, 2, 3)
plt.plot(df['Fibonacci'], df['proof_size_bytes'], marker='o', color='red')
plt.xlabel('Fibonacci')
plt.ylabel('Proof Size (bytes)')
plt.title('Proof Size vs Fibonacci')
plt.xscale('log')

# Plotting 'time_seconds' vs 'Fibonacci'
plt.subplot(2, 2, 4)
plt.plot(df['Fibonacci'], df['time_seconds'], marker='o', color='purple')
plt.xlabel('Fibonacci')
plt.ylabel('Time (seconds)')
plt.title('Time vs Fibonacci')
plt.xscale('log')

plt.tight_layout()
plt.show()

