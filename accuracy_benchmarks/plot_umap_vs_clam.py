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
import pandas as pd
from matplotlib.ticker import FuncFormatter
import seaborn as sns

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

def create_joint_plot_df(dataname, source_name, df, out_path):
    
    min_lengths = df.groupby('Label').size().min()
    # Truncate datasets to match the minimum length for each label
    # df_truncated = df.groupby('Label').apply(lambda x: x[:min_lengths])
    df_truncated = df.groupby('Label', group_keys=True).apply(lambda x: x[:min_lengths])

    plt.ylabel('Y (%)')

# Format y-axis ticks as percentages
   
    plt.figure(figsize=(10, 6))  # Set the figure size
    sns.barplot(x='X', y='Y', hue='Label', data=df_truncated.reset_index(drop=True), edgecolor='black')
    plt.title('Accuracy of ' +  dataname +  " graphs created by " + source_name)
    formatter = FuncFormatter(lambda y, _: '{:.0%}'.format(y / 100))
    plt.gca().yaxis.set_major_formatter(formatter)
    plt.yticks(np.arange(0, 101, 10))
    if source_name == "clam":
        plt.xlabel('min_depth')
    else:
        plt.xlabel('num_neighbors')
    # Add title and labels
    # Save plot to image file
    plt.ylabel('')
    plt.legend(loc='center left', bbox_to_anchor=(1, 0.5))
    plt.savefig(out_path, bbox_inches='tight', dpi=300)

def create_joint_line_plot_df(dataname, source_name, df, out_path):
    min_lengths = df.groupby('Label').size().min()
    # Truncate datasets to match the minimum length for each label
    # df_truncated = df.groupby('Label').apply(lambda x: x[:min_lengths])
    df_truncated = df.groupby('Label', group_keys=True).apply(lambda x: x[:min_lengths])
    print(df_truncated)
    min_y_value = df_truncated['Y'].min()
    min_y_label = df_truncated[df_truncated['Y'] == min_y_value]['Label'].iloc[0]
    print("Minimum Y value:", min_y_value)
    print("Label corresponding to the minimum Y value:", min_y_label)

    plt.figure(figsize=(10, 6))  # Set the figure size
    sns.lineplot(x='X', y='Y', hue='Label', data=df_truncated.reset_index(drop=True), alpha=0.2)
    sns.scatterplot(x='X', y='Y', hue='Label', data=df_truncated.reset_index(drop=True), marker='o', s=100, legend=False)
    
    plt.title('Accuracy of ' +  dataname +  " graphs created by " + source_name)
    formatter = FuncFormatter(lambda y, _: '{:.0%}'.format(y / 100))
    plt.gca().yaxis.set_major_formatter(formatter)
    plt.yticks(np.arange(0, 101, 10))
    
    if source_name == "clam":
        plt.xlabel('min_depth')
    else:
        plt.xlabel('num_neighbors')
    plt.grid(True) 
    
    plt.ylabel('')
    plt.legend(loc='center left', bbox_to_anchor=(1, 0.5))
    plt.savefig(out_path, bbox_inches='tight', dpi=300)


def create_joint_plot(dataname, source_name, test_results_dict, out_path):
    # Extract keys and values from the depth dictionary
    # Iterate through the outer dictionary

    # Add title and labels
    plt.title('Accuracy of ' +  dataname +  " graphs created by " + source_name)
    if source_name == "clam":
        plt.xlabel('min_depth')
    else:
        plt.xlabel('num_neighbors')

    plt.ylabel('Accuracy')
    plt.grid(True)
    plt.gca().yaxis.set_major_formatter(mtick.PercentFormatter(xmax=100.0))
    plt.ylim(0, 100)

    max_x_values = [max(map(int, inner_dict.keys())) for inner_dict in test_results_dict.values()]
    min_x_values = [min(map(int, inner_dict.keys())) for inner_dict in test_results_dict.values()]
    min_min_x_ = min(min_x_values)

    min_x = min(max_x_values)
    # print(max_x_values)
    # print(min_x)
    plt.xlim(min_min_x_, min_x)

    # Set the y-axis ticks at intervals of 10%
    plt.yticks(range(0, 101, 10))

    labels = {"edge_distortion" : "edge_accuracy", "angle_distortion" : "angle_accuracy", "edge_equivalence" : "triangle_equivalence"}
    group_width = 0.25
    
    # Save plot to file
    for key, inner_dict in test_results_dict.items():
    # Sort the inner dictionary by keys and overwrite it
        sorted_keys = sorted(map(int, inner_dict.keys()))
        # print("sorted keys ", sorted_keys)
        # Extract corresponding values based on sorted keys
        sorted_values = [inner_dict[str(k)] for k in sorted_keys]
        if key == "edge_distortion" or key == "angle_distortion":
            sorted_values = [((1.0 - val) * 100) for val in sorted_values]

        else:
            sorted_values = [val * 100 for val in sorted_values]
         # Plot the sorted keys and corresponding values
        plt.plot(sorted_keys, sorted_values, label=labels.get(key, key))
        plt.scatter(sorted_keys, sorted_values, label=labels.get(key, key))

    # plt.legend()
    # Move the legend outside the plot area to the right
    plt.legend(loc='center left', bbox_to_anchor=(1, 0.5))
    plt.savefig(out_path, bbox_inches='tight', dpi=300)

def data_to_df(results):
    results2 = dict()
    labels = {"edge_distortion" : "edge_accuracy", "angle_distortion" : "angle_accuracy", "edge_equivalence" : "triangle_equivalence"}

    for key, inner_dict in results.items():
        # Sort the inner dictionary by keys and overwrite it
        sorted_keys = sorted(map(int, inner_dict.keys()))
        # print("sorted keys ", sorted_keys)

        # Extract corresponding values based on sorted keys
        sorted_values = [inner_dict[str(k)] for k in sorted_keys]

        # Transform values based on conditions
        if key == "edge_distortion" or key == "angle_distortion":
            if key == "angle_distortion":
                print([(val) for val in sorted_values])
                print('------')
                print([(1.0 - val) for val in sorted_values])
                print('------')

                print([(1.0 - val) * 100 for val in sorted_values])
                print('------')

            sorted_values = [(1.0 - (val / 3.0)) * 100 for val in sorted_values]
        else:
            sorted_values = [val * 100 for val in sorted_values]

        # Store sorted keys and values in results2 dictionary as a 2D array
        results2[labels[key]] = [sorted_keys, sorted_values]
    # df = pd.DataFrame.from_dict(results, orient='index')
    # print(results2)
    data = []

    # Iterate through results2 dictionary
    for label, (x_values, y_values) in results2.items():
        for x, y in zip(x_values, y_values):
            data.append((x, y, label))

    # Create DataFrame from the list
    df = pd.DataFrame(data, columns=['X', 'Y', 'Label'])
    return df


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python script.py dataname test_name<1..n>")
    if len(sys.argv) == 3:
        dataname = sys.argv[1]
        source_name = sys.argv[2]
        tests = ["edge_equivalence", "edge_distortion", "angle_distortion"]
        root_path = pathlib.Path("../clam_ffi/clam_ffi/accuracy_results/")
        if source_name == "umap":
            tests = ["umap_" + test for test in tests]
        # data_paths = [root_path /equivalence_test / dataname, root_path / edge_distortion_test/ dataname, root_path / angle_distortion_test/ dataname ]
        out_path = pathlib.Path("plots/" + "all_tests" + "/")
        if source_name == "umap":
            out_path = pathlib.Path("umap_plots/" + "all_tests/")

        if not os.path.exists(out_path):
            os.makedirs(out_path)
        if source_name == "umap":
            raw_data = {test : read_csv_file(pathlib.Path(root_path / test / (str(dataname) + ".csv"))) for test  in tests}
            results = dict()
            for test, data in raw_data.items():
                # print(data)
                # print()
                processed_data = {str(int(data[0][i])) : data[1][i] for i in range(len(data[0]))}
                # print(processed_data)
                results[test.replace("umap_","")] = processed_data
                # print(results)

            df = data_to_df(results)
            create_joint_line_plot_df(dataname, source_name, df, pathlib.Path(str(out_path) + "/" + dataname + ".png"))
            # create_joint_plot(dataname, source_name, results, pathlib.Path(str(out_path) + "/" + dataname + ".png"))
            # df = data_to_df(results)
            # create_joint_plot_df(dataname, source_name, df, pathlib.Path(str(out_path) + "/" + dataname + ".png"))

        else:
            results = {test : process_directory(root_path / test / dataname) for test in tests}
            # create_joint_plot(dataname, source_name, results, pathlib.Path(str(out_path) + "/" + dataname + ".png"))

            df = data_to_df(results)
            create_joint_line_plot_df(dataname, source_name, df, pathlib.Path(str(out_path) + "/" + dataname + ".png"))
            # Find the minimum length for each label
           
    # else:    
    #     dataname = sys.argv[1]
    #     testname = sys.argv[2]

       

    #     root_path = pathlib.Path("../clam_ffi/clam_ffi/accuracy_results/")
    #     test_path = root_path / str(testname)
    #     data_path = test_path / dataname
    #     out_folder = pathlib.Path("plots/" + testname + "/")
    #      # Check if the folder exists
    #     if not os.path.exists(out_folder):
    #         # Create the folder if it doesn't exist
    #         os.makedirs(out_folder)
    #     depth_dict = process_directory(data_path)
    #     create_scatterplot_from_dict(dataname, testname, depth_dict, str(out_folder) + '/' + dataname)

        