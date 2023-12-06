import json
import pandas as pd
import matplotlib.pyplot as plt

# Load the JSON data from the file
file_path = 'benchmarks/miden/fibonacci/verify_bench.json'  # Replace with the path to your JSON file

with open(file_path, 'r') as file:
    data = json.load(file)

# Extracting data for visualization
results = data['results'][0]['results']  # Focusing on the first entry in 'results', i.e., 'Fibonacci'

# Creating a DataFrame
df = pd.DataFrame({
    'name': [int(r['name']) for r in results],  # converting names to integers
    'cycles': [r['metrics']['cycles'] for r in results],
    'memory_usage_bytes': [r['metrics']['memory_usage_bytes'] for r in results],
    'proof_size_bytes': [r['metrics']['proof_size_bytes'] for r in results]
})

# Creating visualizations
plt.figure(figsize=(15, 5))

# Plotting 'cycles' vs 'name'
plt.subplot(1, 3, 1)
plt.plot(df['name'], df['cycles'], marker='o')
plt.xlabel('Name')
plt.ylabel('Cycles')
plt.title('Cycles vs Name')
plt.xscale('log')

# Plotting 'memory_usage_bytes' vs 'name'
plt.subplot(1, 3, 2)
plt.plot(df['name'], df['memory_usage_bytes'], marker='o', color='green')
plt.xlabel('Name')
plt.ylabel('Memory Usage (bytes)')
plt.title('Memory Usage vs Name')
plt.xscale('log')

# Plotting 'proof_size_bytes' vs 'name'
plt.subplot(1, 3, 3)
plt.plot(df['name'], df['proof_size_bytes'], marker='o', color='red')
plt.xlabel('Name')
plt.ylabel('Proof Size (bytes)')
plt.title('Proof Size vs Name')
plt.xscale('log')

plt.tight_layout()
plt.show()

