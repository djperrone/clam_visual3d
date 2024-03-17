import numpy as np
import matplotlib.pyplot as plt
from sklearn.datasets import fetch_openml
from sklearn.preprocessing import StandardScaler
import umap
import csv

# Load the MNIST dataset
mnist = fetch_openml('mnist_784', version=1)
X, y = mnist['data'], mnist['target']

# Scale the data
scaler = StandardScaler()
X_scaled = scaler.fit_transform(X)

# Perform UMAP projection into 3D space
for i in range(5, 20):
    n = i
    umap_model = umap.UMAP(n_components=3, n_neighbors=n)  # Setting n_neighbors to 15
    embedding = umap_model.fit_transform(X_scaled)

    # Save the 3D coordinates to a CSV file
    output_file = "mnist_3d_umap_projection_n" + str(n) + ".csv"
    with open(output_file, 'w', newline='') as csvfile:
        writer = csv.writer(csvfile)
        writer.writerow(['X', 'Y', 'Z', 'Digit'])  # Header
        for i in range(len(embedding)):
            writer.writerow([embedding[i, 0], embedding[i, 1], embedding[i, 2], y[i]])

    # Plot the 3D UMAP projection
    fig = plt.figure(figsize=(10, 8))
    ax = fig.add_subplot(111, projection='3d')

    # Scatter plot points colored by their true labels
    for i in range(10):
        ax.scatter(embedding[y == str(i), 0], embedding[y == str(i), 1], embedding[y == str(i), 2], label=str(i), s=5)

    ax.set_title('3D UMAP Projection of MNIST Dataset')
    ax.legend(title='Digit')
    plt.show()
