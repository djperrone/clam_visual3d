import sys
import csv
import matplotlib.pyplot as plt
import os
import pandas as pd

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

def convert_to_rolling_mean(column_data, window_size):
    rolling_means = []
    for j in range(len(column_data) - window_size + 1):
        window = column_data[j:j+window_size]
        mean = sum(window) / window_size
        rolling_means.append(mean)
    return rolling_means


def plot_line_graph(outfile, column_averages):
    plt.scatter(range(1, len(column_averages) + 1), column_averages, marker='o')
    plt.xlabel('Column Number')
    plt.ylabel('Average Value (Accuracy)')
    plt.title(f'Average Value of iteration - {filename_without_extension}')
    plt.grid(True)
    plt.show()
    print("saving plot to ", outfile)
    plt.savefig(outfile)  # Save the plot to the specified file
    plt.close()  # Close the plot to release memory


def plot_line_graph_pd(outfile, df):
    # plt.scatter(range(1, len(df['Column_Averages']) + 1), df['Column_Averages'], marker='o')
    # # plt.plot(df['rolling_sales_5'], label='Rolling Mean')
    # # plt.plot(df['Column_Averages'], label='Raw Data')
    # plt.plot(df['Column_Averages_rolling_10'], label='Rolling Mean')

    fig, ax = plt.subplots(figsize = (16,10))
    plt.scatter(range(1, len(df['Column_Averages']) + 1), df['Column_Averages'], marker='o', label='Raw Data', s = 0.5)
    plt.plot(df['Column_Averages_rolling_10'], label='Rolling Mean', color='red', linestyle='-', linewidth=1.0)  # Adjust color and style as needed
    plt.xlabel('Time Step')
    plt.ylabel('Triangle Accuracy')
    plt.title(f'{filename_without_extension}')
    plt.legend()
    plt.grid(True)
    plt.show()
    print("saving plot to ", outfile)
    plt.savefig(outfile, dpi=300, bbox_inches="tight")  # Save the plot to the specified file
    plt.close()  # Close the plot to release memory

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script.py filename.csv")
    else:
        print("running")
        filename = sys.argv[1]
        try:
            data = read_csv_file(filename)
            if not data:
                print("No data found in the file.")
            else:
                column_averages = calculate_column_averages(data)
                 # Convert average values to rolling means with window size 10
                # rolling_means = convert_to_rolling_mean(column_averages, 10)
                # print(len(column_averages))
                # print(len(rolling_means))
                # Create a dictionary where keys are column numbers and values are the average values of each column
                # data_dict = {f"Column {i+1}": column for i, column in enumerate(column_averages)}

# Load the data dictionary into a Pandas DataFrame
                # df = pd.DataFrame(column_averages)

                df = pd.DataFrame({'Column_Averages': column_averages})               
                df['Column_Averages_rolling_10'] = df['Column_Averages'].rolling(80).mean()
                # print(df)
                # rolling_means = df.rolling(window=10).mean()
                # print("Average value of each column:", column_averages)
                filename_without_extension = os.path.splitext(os.path.basename(filename))[0]
                plot_line_graph_pd(filename_without_extension, df)
        except FileNotFoundError:
            print("File not found:", filename)
