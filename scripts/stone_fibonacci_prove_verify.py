import json
import matplotlib.pyplot as plt

# Path to the JSON file
file_path = "../stone_stark/benchmarks/fibonacci.json"  # Replace with your file path

# Reading JSON data from the file
with open(file_path, "r") as file:
    json_data = json.load(file)

# Parsing data
names = []
times = []
units = []

for item in json_data:
    for key in item:
        names.append(key)
        times.append(item[key]["time_taken"])
        units.append(item[key]["time_unit"])

# Adjusting time to a common unit (e.g., seconds)
# Assuming 's' is for seconds and 'ns' is for nanoseconds
adjusted_times = []
for time, unit in zip(times, units):
    if unit == "s":
        adjusted_times.append(time)
    elif unit == "ns":
        adjusted_times.append(time * 1e-9)  # Converting nanoseconds to seconds

# Creating the bar chart
plt.figure(figsize=(8, 6))
plt.bar(names, adjusted_times, color=["blue", "orange"])
plt.xlabel("Step")
plt.ylabel("Time Taken (s)")
plt.title("Time Taken for Proving and Verifying (10-fib)")
plt.yscale("log")  # Using logarithmic scale due to large differences in time scales
plt.show()
