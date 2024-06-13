import os
import re
import sys
import shutil
from datetime import datetime
import pathlib
import platform
import numpy as np
from sklearn.datasets import fetch_openml
from sklearn.model_selection import train_test_split
import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.image as mpimg

PROJECT_PATH = pathlib.Path(__file__).parent.parent.resolve()

DATA_PATH = PROJECT_PATH / "data/anomaly_data/preprocessed"
DATA_PATH.mkdir(exist_ok=True)

print("downloading mnist dataset")
# Download MNIST dataset from scikit-learn
# mnist = fetch_openml('mnist_784', version=1, cache=True)
X, y = fetch_openml("mnist_784", version=1, return_X_y=True, as_frame=False)
print("done downloading. Converting to expected datatype")

label_counts = np.bincount(y.astype(int))

# Plot the bar plot
plt.bar(range(len(label_counts)), label_counts)
plt.xlabel('Labels')
plt.ylabel('Count')
plt.title('Distribution of Labels in Training Data')
plt.savefig("reg", dpi=300, bbox_inches="tight")  # Save the plot to the specified file


X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.95, random_state=42)
print(set(y))
print(type(y))
print(type(y_train))
X = X_train
y = y_train
print(set(y))

label_counts = np.bincount(y.astype(int))

# Plot the bar plot
plt.bar(range(len(label_counts)), label_counts)
plt.xlabel('Labels')
plt.ylabel('Count')
plt.title('Distribution of Labels in Training Data')
plt.savefig("ds", dpi=300, bbox_inches="tight")  # Save the plot to the specified file

# Separate features and labels
# X, y = mnist['data'], mnist['target']
X = X.astype(np.float32) / np.float32(255.0)

# Convert labels to integers
y = y.astype(np.uint8)
print("Finished converting. Saving features and scores")
print("Dimensions of images:", X.shape)
print("Dimensions of labels:", y.shape)

# Save features and labels into .npy files
features_path = DATA_PATH / 'down-mnist_features.npy'
scores_path = DATA_PATH / 'down-mnist_scores.npy'
np.save(features_path, X)
np.save(scores_path, y)

print("Finished mnist setup")



# from sklearn.datasets import fetch_openml
# import os
# import re
# import sys
# import shutil
# from datetime import datetime
# import pathlib
# import platform
# import numpy as np
# from sklearn.datasets import fetch_openml
# import tensorflow as tf
# import seaborn as sns
# import numpy as np
# import pandas as pd
# import matplotlib.pyplot as plt
# import matplotlib.image as mpimg

# PROJECT_PATH = pathlib.Path(__file__).parent.parent.resolve()

# DATA_PATH = PROJECT_PATH / "data/anomaly_data/preprocessed"
# DATA_PATH.mkdir(exist_ok=True)

# # Download MNIST dataset from scikit-learn
# # mnist = fetch_openml('mnist_784', version=1, cache=True)
# X, y = fetch_openml("mnist_784", version=1, return_X_y=True, as_frame=False)
# print("done downloading. Converting to expected datatype")

# label_counts = np.bincount(y.astype(int))

# # Plot the bar plot
# plt.bar(range(len(label_counts)), label_counts)
# plt.xlabel('Labels')
# plt.ylabel('Count')
# plt.title('Distribution of Labels in Training Data')
# plt.savefig("reg1", dpi=300, bbox_inches="tight")  # Save the plot to the specified file




# # Split the dataset into training and testing sets
# # X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.5, random_state=42)

# # Count the occurrences of each label
# # label_counts = np.bincount(y_train.astype(int))

# # # Plot the bar plot
# # plt.bar(range(len(label_counts)), label_counts)
# # plt.xlabel('Labels')
# # plt.ylabel('Count')
# # plt.title('Distribution of Labels in Training Data')
# # plt.savefig("ds1", dpi=300, bbox_inches="tight")  # Save the plot to the specified file




# # Optionally, you can downsample the test dataset as well using a similar approach

# # Now you can use X_train_downsampled, y_train_downsampled, X_test, and y_test in your model training
# X = X.astype(np.float32) / np.float32(255.0)

# # Convert labels to integers


# y = y.astype(np.uint8)
# # print(y_train_downsampled)
# # Count the occurrences of each label
# # label_counts = np.bincount(y_train_downsampled.astype(int))

# # Plot the bar plot
# plt.bar(range(len(label_counts)), label_counts)
# plt.xlabel('Labels')
# plt.ylabel('Count')
# plt.title('Distribution of Labels in Training Data')
# plt.savefig("ds", dpi=300, bbox_inches="tight")  # Save the plot to the specified file


# # Save features and labels into .npy files
# features_path = DATA_PATH / 'mnist-downsampled_features.npy'
# scores_path = DATA_PATH / 'mnist-downsampled_scores.npy'
# np.save(features_path, X)
# np.save(scores_path, y)

# print("Finished mnist setup")