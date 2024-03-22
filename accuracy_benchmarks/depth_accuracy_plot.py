import matplotlib.pyplot as plt

import os
import matplotlib.ticker as mtick
import numpy as np
import matplotlib.pyplot as plt
import umap
import csv
import os
import sys
from datetime import datetime
import pathlib
import platform
import numpy as np

def extract_data_from_filename(filename):
    # Extract the filename from the path
    filename = filename.split("/")[-1]

    # Split the filename by "_"
    filename_parts = filename.split("_")

    # Extract the dataname (first part)
    dataname = filename_parts[0]

    # Extract depthvalue by splitting and stripping ".csv" extension
    depthvalue = filename_parts[-1].split(".csv")[0]

    return dataname, depthvalue

def read_csv_file(filename):
    data = []
    with open(filename, 'r') as file:
        reader = csv.reader(file)
        for row in reader:
            if row:  # Check if the row is not empty
                data.append([float(val) for val in row])  # Convert non-empty values to float
    return data

def calculate_column_averages(data):
    num_columns = len(data[0])
    column_sums = [0] * num_columns
    column_counts = [0] * num_columns

    for row in data:
        for i, val in enumerate(row):
            column_sums[i] += val
            column_counts[i] += 1

    column_averages = [column_sum / column_count for column_sum, column_count in zip(column_sums, column_counts)]
    return column_averages

def process_directory(directory):
    depth_max_dict = {}  # Initialize the dictionary to store depth and max value pairs

    # Iterate through every file in the directory
    for filename in os.listdir(directory):
        # Check if the file is a CSV file
        if filename.endswith(".csv"):
            csv_file = os.path.join(directory, filename)
            dataname, depthvalue = extract_data_from_filename(csv_file)
            print(csv_file, ' ', filename)
            data = read_csv_file(csv_file)
            averages = calculate_column_averages(data)

            # Update the dictionary with depth and max value pairs
            if depthvalue not in depth_max_dict:
                depth_max_dict[depthvalue] = averages[len(averages)-1]
            else:
                print("test")
                depth_max_dict[depthvalue] = max(depth_max_dict[depthvalue], averages[len(averages)-1])

    return depth_max_dict
def create_scatterplot_from_dict(dataname,testname, depth_max_dict, outfile):
    # Extract keys and values from the depth dictionary
    depths = list(depth_max_dict.keys())
    depths = [int(depth) for depth in depths]
    max_values = list(depth_max_dict.values())

    print(depths)
    print(max_values)
    max_values = [val * 100 for val in max_values]

    # Create scatter plot
    plt.figure(figsize=(8, 6))
    plt.scatter(depths, max_values, color='blue', marker='o')
    plt.title(dataname + ' ' + testname+ ' vs Depth')
    plt.xlabel('Depth')
    plt.ylabel('Accuracy')
    plt.grid(True)
    plt.gca().yaxis.set_major_formatter(mtick.PercentFormatter(xmax=100.0))
    plt.ylim(0, 100)
    # plt.legend(loc='upper right') 
    # Set the y-axis ticks at intervals of 10%
    plt.yticks(range(0, 101, 10))

    plt.show()
    print("out ", outfile)
    plt.savefig(outfile)  # Save the plot to the specified file
    plt.close()  # Close the plot to release memory

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python script.py dataname test_name<1..n>")
    else:    
        dataname = sys.argv[1]
        testname = sys.argv[2]


        root_path = pathlib.Path("../clam_ffi/clam_ffi/accuracy_results/")
        test_path = root_path / str(testname)
        data_path = test_path / dataname
        out_folder = pathlib.Path("plots/" + testname + "/")
         # Check if the folder exists
        if not os.path.exists(out_folder):
            # Create the folder if it doesn't exist
            os.makedirs(out_folder)
        depth_dict = process_directory(data_path)
        create_scatterplot_from_dict(dataname, testname, depth_dict, str(out_folder) + '/' + dataname)

        