import json
import matplotlib.pyplot as plt

# Replace with the path to your JSON file
file_path = '../benchmarks/risczero/fibonacci/bench.json'

# Reading JSON data from the file
with open(file_path, 'r') as file:
    data = json.load(file)

# Extracting the results for Fibonacci
fibonacci_results = data['results'][0]['results']

# Parsing data for proving and verifying
proving_times = []
verifying_times = []
fib_names = []

for result in fibonacci_results:
    fib_names.append(result['name'])
    for metric in result['metrics']:
        if metric['func'] == 'proving':
            # Converting proving time to seconds
            proving_times.append(float(metric['time']) / 1000)
        elif metric['func'] == 'verifying':
            # Keeping verifying time in nanoseconds
            verifying_times.append(float(metric['time']))

# Creating line graphs
plt.figure(figsize=(12, 6))

# Proving time graph
plt.subplot(1, 2, 1)
plt.plot(fib_names, proving_times, marker='o', color='blue')
plt.xlabel('Fibonacci Sequence')
plt.ylabel('Time (s)')
plt.title('Proving Time for Fibonacci Sequences')

# Verifying time graph
plt.subplot(1, 2, 2)
plt.plot(fib_names, verifying_times, marker='o', color='orange')
plt.xlabel('Fibonacci Sequence')
plt.ylabel('Time (ns)')
plt.title('Verifying Time for Fibonacci Sequences')

plt.tight_layout()
plt.show()

