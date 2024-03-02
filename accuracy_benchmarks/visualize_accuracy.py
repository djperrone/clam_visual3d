import sys
import csv
import matplotlib.pyplot as plt
import matplotlib.ticker as mtick
import os
import pandas as pd
import pathlib

def read_csv_file(filename):
    data = []
    with open(filename, 'r') as file:
        reader = csv.reader(file)
        for row in reader:
            if row:  # Check if the row is not empty
                data.append([float(val) for val in row])  # Convert non-empty values to float
    return data

# def read_descriptor_file(filename):
#     data = []
#     with open(filename, 'r') as file:
#         reader = csv.reader(file)
#         for row in reader:
#             if row:  # Check if the row is not empty
#                 data.append([val for val in row])  # Convert non-empty values to float
#     return data

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

def convert_to_rolling_mean(column_data, window_size):
    rolling_means = []
    for j in range(len(column_data) - window_size + 1):
        window = column_data[j:j+window_size]
        mean = sum(window) / window_size
        rolling_means.append(mean)
    return rolling_means


# def plot_line_graph(outfile, column_averages):
#     plt.scatter(range(1, len(column_averages) + 1), column_averages, marker='o')
#     plt.xlabel('Column Number')
#     plt.ylabel('Average Value (Accuracy)')
#     plt.title(f'Average Value of iteration - {filename_without_extension}')
#     plt.grid(True)
#     plt.show()
#     print("saving plot to ", outfile)
#     plt.savefig(outfile)  # Save the plot to the specified file
#     plt.close()  # Close the plot to release memory


def plot_line_graph_pd(title, outpath, df, descriptor_text):

    fig, ax = plt.subplots(figsize = (16,10))
    plt.scatter(range(1, len(df['Column_Averages']) + 1), df['Column_Averages'] * 100, marker='o', label='Raw Data', s = 0.5)
    plt.plot(df['Column_Averages_rolling_10'] * 100, label='Rolling Mean', color='red', linestyle='-', linewidth=1.0)  # Adjust color and style as needed
    plt.xlabel('Time Step')
    plt.ylabel('Triangle Accuracy')
    plt.title(f'{title}')
    plt.gca().yaxis.set_major_formatter(mtick.PercentFormatter(xmax=100.0))
    plt.ylim(0, 100)
    plt.text(0.8, 0.8, descriptor_text, transform=plt.gca().transAxes)
    plt.legend(loc='upper right') 
    # Set the y-axis ticks at intervals of 10%
    plt.yticks(range(0, 101, 10))

    # # Set y-axis limits and formatter
    plt.legend()
    plt.grid(True)
    plt.show()
    print("saving plot to ", outpath)
    plt.savefig(outpath, dpi=300, bbox_inches="tight")  # Save the plot to the specified file
    plt.close()  # Close the plot to release memory

def plot2(title, outpath, df1, df2):
    fig, axs = plt.subplots(1, 2, figsize=(16, 10))  # Create two subplots side by side
    print("p2")
    # Plot for DataFrame 1
    axs[0].scatter(range(1, len(df1['Column_Averages']) + 1), df1['Column_Averages'] * 100, marker='o', label='Raw Data', s=0.5)
    axs[0].plot(df1['Column_Averages_rolling_10'] * 100, label='Rolling Mean', color='red', linestyle='-', linewidth=1.0)
    axs[0].set_xlabel('Time Step')
    axs[0].set_ylabel('Triangle Distortion')
    axs[0].set_title(f'{title} - Edge Distortion')
    axs[0].yaxis.set_major_formatter(mtick.PercentFormatter(xmax=100.0))
    axs[0].set_ylim(0, 100)
    axs[0].set_yticks(range(0, 101, 10))  # Set y-axis ticks at intervals of 10%
    axs[0].legend()
    axs[0].grid(True)

    # Plot for DataFrame 2
    axs[1].scatter(range(1, len(df2['Column_Averages']) + 1), df2['Column_Averages'] * 100, marker='o', label='Raw Data', s=0.5)
    axs[1].plot(df2['Column_Averages_rolling_10'] * 100, label='Rolling Mean', color='red', linestyle='-', linewidth=1.0)
    axs[1].set_xlabel('Time Step')
    axs[1].set_ylabel('Triangle Accuracy')
    axs[1].set_title(f'{title} -  Triangle Equivalence')
    axs[1].yaxis.set_major_formatter(mtick.PercentFormatter(xmax=100.0))
    axs[1].set_ylim(0, 100)
    axs[1].set_yticks(range(0, 101, 10))  # Set y-axis ticks at intervals of 10%
    axs[1].legend()
    axs[1].grid(True)

    plt.tight_layout()  # Adjust layout to prevent overlap

    # Save the plot to the specified file
    print("saving plot to ", outpath)
    plt.savefig(outpath, dpi=300, bbox_inches="tight")  
    plt.close()  # Close the plot to release memory



def plot_for_each_in_dir(in_folder, out_folder):
    # Iterate through each file in the folder
    print(out_folder)
    for file_path in in_folder.iterdir() :
        if file_path.is_file() and file_path.suffix == ".csv":
            print(file_path)
            data = read_csv_file(file_path)
            if not data:
                print("No data found in the file.")
            else:
                column_averages = calculate_column_averages(data)

                df = pd.DataFrame({'Column_Averages': column_averages})               
                df['Column_Averages_rolling_10'] = df['Column_Averages'].rolling(80).mean()

                descriptor_file = os.path.splitext(file_path)[0] + '.txt'  # Replace .csv with .txt
                descriptors = extract_descriptor(descriptor_file)
                descriptor_text = ""
                for column_name, value in descriptors.items():
                    descriptor_text += f"{column_name}: {value}\n"
                # Add a text box with additional information
                graph_size = descriptors['graph_vertex_cardinality']
                data_size = descriptors['data_cardinality']
                ratio = float((graph_size) / float(data_size)) * 100
                ratio = "{:.2f}".format(float(ratio))
                descriptor_text += "graph/data ratio: " + ratio + "%"
                # print(df)
                # rolling_means = df.rolling(window=10).mean()
                # print("Average value of each column:", column_averages)
                title = os.path.basename(file_path)
                filename = os.path.splitext(os.path.basename(file_path))[0]
                out_path = out_folder / filename
                # plt.text(3, 6, "testing1234", bbox=dict(facecolor='white', alpha=0.5))
                plot_line_graph_pd(title, out_path, df, descriptor_text)

def create_pd_df(data):
    if not data:
        print("No data found in the file.")
    else:
        column_averages = calculate_column_averages(data)

        df = pd.DataFrame({'Column_Averages': column_averages})               
        df['Column_Averages_rolling_10'] = df['Column_Averages'].rolling(80).mean()
        return df

import os



def extract_descriptor(filename):
    # Read the data from the file
    with open(filename, 'r') as file:
        # Read the first line for column names and split it into a list
        columns = file.readline().strip().split(',')

        # Read the second line for integer values and split it into a list
        values = file.readline().strip().split(',')

        # Convert the values to integers
        values = [int(val) for val in values]

        # Create a dictionary where keys are column names and values are integer values
        data_dict = {column: value for column, value in zip(columns, values)}

        # Create a DataFrame from the dictionary
        # df = pd.DataFrame(data_dict)
        return data_dict


def read_descriptor_txt_from_csv(file_name):
    """
    Read data from a CSV file if it exists, otherwise read data from a text file with the same name by replacing the extension with '.txt'.
    
    Parameters:
        file_name (str): The name of the file to read from, initially with the '.csv' extension.
        
    Returns:
        list: A list containing the data read from the file, or an empty list if the file does not exist.
    """
    csv_file = file_name  # Original CSV file name
    txt_file = os.path.splitext(file_name)[0] + '.txt'  # Replace .csv with .txt

    if os.path.exists(txt_file):
        # Read data from the text file
        with open(txt_file, 'r') as file:
            data = file.readlines()
        return data
    else:
        print(f"Neither '{csv_file}' nor '{txt_file}' exists.")
        return []




def plot_for_each_in_dir2(in_folder1, in_folder2, out_folder):
    dir1_files = sorted([file for file in in_folder1.iterdir() if file.is_file() and file.suffix == ".csv"])
    dir2_files = sorted([file for file in in_folder2.iterdir() if file.is_file() and file.suffix == ".csv"])
    for file1, file2 in zip(dir1_files, dir2_files):
        print(file1, file2)
        data1 = read_csv_file(file1)
        print("here1")

        data2 = read_csv_file(file2)
        print("here2")
        df1 = create_pd_df(data1)
        df2 = create_pd_df(data2)
        title = os.path.basename(file1)
        filename = os.path.splitext(os.path.basename(file1))[0]
        out_path = out_folder / filename
        print("here")
        plot2(title, out_path, df1, df2)



if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python script.py dataname test_name<1..n>")
    elif len(sys.argv) < 4:
        dataname = sys.argv[1]
        testname = sys.argv[2]


        root_path = pathlib.Path("../clam_ffi/clam_ffi/accuracy_results/")
        test_path = root_path / testname
        data_path = test_path / dataname
        out_folder = pathlib.Path("plots/" + testname)

        # Check if the folder exists
        if not os.path.exists(out_folder):
            # Create the folder if it doesn't exist
            os.makedirs(out_folder)

        print("running 1")
        try:
            plot_for_each_in_dir(data_path, out_folder)

        except FileNotFoundError:
            # print("File not found:", filename)
            print()

    elif len(sys.argv) < 5:
        print("running all")
        dataname = sys.argv[1]
        testname1 = sys.argv[2]
        testname2 = sys.argv[3]


        root_path = pathlib.Path("../clam_ffi/clam_ffi/")
        test_path1 = root_path / testname1
        data_path1 = test_path1 / dataname
        out_folder = pathlib.Path("results")
        test_path2 = root_path / testname2
        data_path2 = test_path2 / dataname

        try:
            # plot_for_each_in_dir(data_path, out_folder)
            plot_for_each_in_dir2(data_path1, data_path2, out_folder)

        except FileNotFoundError:
            # print("File not found:", filename)
            print()