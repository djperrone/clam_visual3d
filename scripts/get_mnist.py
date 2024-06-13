import os
import re
import sys
import shutil
from datetime import datetime
import pathlib
import platform
import numpy as np
from sklearn.datasets import fetch_openml

PROJECT_PATH = pathlib.Path(__file__).parent.parent.resolve()

DATA_PATH = PROJECT_PATH / "data/anomaly_data/preprocessed"
DATA_PATH.mkdir(exist_ok=True)

print("downloading mnist dataset")
# Download MNIST dataset from scikit-learn
# mnist = fetch_openml('mnist_784', version=1, cache=True)
X, y = fetch_openml("mnist_784", version=1, return_X_y=True, as_frame=False)
print("done downloading. Converting to expected datatype")

# Separate features and labels
# X, y = mnist['data'], mnist['target']
X = X.astype(np.float32) / np.float32(255.0)

# Convert labels to integers
y = y.astype(np.uint8)
print("Finished converting. Saving features and scores")

# Save features and labels into .npy files
features_path = DATA_PATH / 'mnist_features.npy'
scores_path = DATA_PATH / 'mnist_scores.npy'
np.save(features_path, X)
np.save(scores_path, y)

print("Finished mnist setup")
