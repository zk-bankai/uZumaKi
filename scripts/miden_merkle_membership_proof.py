import json
import pandas as pd
import matplotlib.pyplot as plt

# Load the JSON data from the file
file_path = 'benchmarks/miden/MerkleTree/MerkleMembership/bench.json'  # Replace with the path to your JSON file

with open(file_path, 'r') as file:
    data = json.load(file)

# Extracting data for visualization
result = data['results'][0]['results'][0]  # Focusing on the single entry under 'Merkle Membership'

# Creating a DataFrame
df = pd.DataFrame({
    'Metric': ['Compressed Proof Size (bytes)', 'Cycles', 'Memory Usage (bytes)', 'Proof Size (bytes)'],
    'Value': [
        result['metrics']['compressed_proof_size_bytes'],
        result['metrics']['cycles'],
        result['metrics']['memory_usage_bytes'],
        result['metrics']['proof_size_bytes']
    ]
})

# Creating a bar chart visualization
plt.figure(figsize=(10, 6))
plt.bar(df['Metric'], df['Value'], color=['blue', 'green', 'red', 'purple'])
plt.xlabel('Metric')
plt.ylabel('Value')
plt.title('Metrics for Merkle Membership')
plt.xticks(rotation=45)
plt.show()

