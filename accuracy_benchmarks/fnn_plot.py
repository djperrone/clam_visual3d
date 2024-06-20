import matplotlib.pyplot as plt

import os
import matplotlib.ticker as mtick
import numpy as np
import umap
import csv
import os
import sys
from datetime import datetime
import pathlib
import platform
import numpy as np
import pandas as pd
import seaborn as sns

from typing import Dict, Tuple

# example usage: python3 fnn_plot.py arrhythmia f1-score

def process_directory(directory, test_name):

    data_rows = []
    # Iterate through each subfolder (depth directory) in the top-level directory
    for depth_dir in directory.iterdir():
        if depth_dir.is_dir():
            depth_str = depth_dir.name.split('_')[1]  # Split by '_' and get the second part
            depth = int(depth_str)
            
            # Iterate through each subfolder (k directory) within the depth directory
            for k_dir in depth_dir.iterdir():
                if k_dir.is_dir():
                    k_str = k_dir.name.split('_')[1]  # Split by '_' and get the second part
                    k = int(k_str)
                    
                    # Iterate through files in the k directory
                    for file in k_dir.iterdir():
                        if file.is_file() and file.suffix == '.csv' and test_name in file.name.lower():
                            
                            try:
                                # Open the CSV file and read its contents
                                with open(file, 'r') as csv_file:
                                    reader = csv.reader(csv_file)
                                    
                                    # Assuming the CSV has a single row, read the first row
                                    for row in reader:
                                        # Convert empty strings to 0 and convert others to float
                                        f1_scores = [float(value) if value != '' else 0 for value in row]
                                        
                                        # Replace NaN values with 0 (using numpy)
                                        f1_scores = [0 if np.isnan(score) else score for score in f1_scores]
                                        
                                        # Append a tuple of (depth, k, f1_scores) to data_rows list
                                        data_rows.append((depth, k, max(f1_scores)))
                                        break  # Read only the first row
                                    
                            except Exception as e:
                                print(f"Error reading file {file}: {str(e)}")
                        

   #  Iterate through each subfolder (depth directory) in the top-level directory
    df = pd.DataFrame(data_rows, columns=['Depth', 'k', 'F1_Score'])
    df = df[df['k'] <= 19]

    # print(df.head())  # Print the first few rows of the DataFrame for verification
    df.fillna(0, inplace=True)
    df_sorted = df.sort_values(by=['Depth', 'k'])
    # print(df_sorted.head())  
    return df_sorted


def plot_contour(df, out_file, dataname, test_name):
    # Extract data from DataFrame
# Replace NaN with 0
    df.fillna(0, inplace=True)

# Pivot the DataFrame to prepare for contour plot
    pivot_df = df.pivot(index='Depth', columns='k', values='F1_Score')
    # print("n------------------------\n")
    # print(pivot_df.head())
# Extract values for plotting
    depth_values = pivot_df.index.values
    k_values = pivot_df.columns.values
    f1_scores = pivot_df.values
    # Create meshgrid for contour plot
    K, Depth = np.meshgrid(np.unique(k_values), np.unique(depth_values))

    # Plotting
    plt.figure(figsize=(10, 6))
    contour = plt.contourf(K, Depth, f1_scores, cmap='viridis')
    plt.colorbar(contour, label='F1 Score')
    plt.xlabel('k')
    plt.ylabel('Depth')
    title = "Contour Plot of " + test_name + " for " + dataname
    plt.title(title)
    plt.grid(True)
    plt.tight_layout()
    # plt.show()
    plt.savefig(out_file)
    # print(out_file)

def plot_heatmap(df, out_file, dataname, test_name):
    df.fillna(0, inplace=True)

    # Pivot the DataFrame to prepare for contour plot
    pivot_df = df.pivot(index='Depth', columns='k', values='F1_Score')
    # Plotting the heatmap
    plt.figure(figsize=(10, 8))
    sns.heatmap(pivot_df, annot=True, fmt=".2f", cmap="YlGnBu", cbar_kws={'label': 'F1 Score'},vmin=0, vmax=1)
#     sns.heatmap(pivot_df, annot=True, fmt=".2f", cmap="YlGnBu", cbar_kws={'label': 'F1 Score'})

    title = "Heatmap of " + test_name + " for " + dataname
    plt.title(title)
    plt.xlabel('k')
    plt.ylabel('Depth')

    # Save the heatmap as a PNG file
    df.fillna(0, inplace=True)

# # Pivot the DataFrame to prepare for contour plot
#     pivot_df = df.pivot(index='Depth', columns='k', values='F1_Score')
#     print("n------------------------\n")
#     print(pivot_df.head())
# # Extract values for plotting
#     pivot_df = df.pivot(index='Depth', columns='k', values='F1_Score')

#     # Plotting the heatmap
#     plt.figure(figsize=(10, 8))
#     sns.heatmap(pivot_df, annot=True, fmt=".2f", cmap="YlGnBu", cbar_kws={'label': 'F1 Score'})
#     plt.title('Heatmap of F1 Scores')
#     plt.xlabel('k')
#     plt.ylabel('Depth')


    plt.savefig(out_file)


if __name__ == "__main__":
    
    if len(sys.argv) < 2:
        print("Usage: python script.py dataname test_name<1..n>")
    if len(sys.argv) == 3:
        dataname = sys.argv[1]
        test_name = sys.argv[2]
        root_path = pathlib.Path("../clam_ffi/clam_ffi/accuracy_results/fnn")

        # data_paths = [root_path /equivalence_test / dataname, root_path / edge_distortion_test/ dataname, root_path / angle_distortion_test/ dataname ]
        out_folder = pathlib.Path("plots/" + "fnn/" + dataname +"/")
        if not os.path.exists(out_folder):
            os.makedirs(out_folder)
        
        df = process_directory(root_path / dataname, test_name)
        out_file = out_folder / (test_name + "_heatmap.png")
        # print(out_file)
        # 
        # create_joint_plot(dataname, "", result, "")
        plot_heatmap(df, out_file, dataname, test_name)

   

        # root_path = pathlib.Path("../clam_ffi/clam_ffi/accuracy_results/")
        # # test_path = root_path / str(testname)
        # data_path = test_path / dataname
        # out_folder = pathlib.Path("plots/" + testname + "/")
        #  # Check if the folder exists
        # if not os.path.exists(out_folder):
        #     # Create the folder if it doesn't exist
        #     os.makedirs(out_folder)
        # depth_dict = process_directory(data_path)
        # create_scatterplot_from_dict(dataname, testname, depth_dict, str(out_folder) + '/' + dataname)

        