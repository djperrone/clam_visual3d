import sys
import csv
import matplotlib.pyplot as plt
import os

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

def plot_line_graph(outfile, column_averages):
    plt.scatter(range(1, len(column_averages) + 1), column_averages, marker='o')
    plt.xlabel('Column Number')
    plt.ylabel('Average Value (Accuracy)')
    plt.title(f'Average Value of iteration - {filename_without_extension}')
    plt.grid(True)
    plt.show()
    plt.savefig(outfile)  # Save the plot to the specified file
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
                # print("Average value of each column:", column_averages)
                filename_without_extension = os.path.splitext(os.path.basename(filename))[0]
                plot_line_graph(filename_without_extension, column_averages)
        except FileNotFoundError:
            print("File not found:", filename)
