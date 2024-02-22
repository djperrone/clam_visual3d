import os
import requests
import json

import os
import re
import sys
import shutil
from datetime import datetime
import pathlib
import platform
import numpy as np

PROJECT_PATH = pathlib.Path(__file__).parent.parent.resolve()

DATA_PATH = PROJECT_PATH / "data"
DATA_PATH.mkdir(exist_ok=True)

SRC_PATH = DATA_PATH / "anomaly_data/classes"
assert SRC_PATH.exists(), "data src not found at " + str(SRC_PATH)

DEST_PATH = DATA_PATH / "anomaly_data"
DEST_PATH.mkdir(exist_ok=True)

def download_dataset(url, path):
    # Download the dataset
    response = requests.get(url)
    with open(path, 'wb') as f:
        f.write(response.content)
    print(f"Dataset downloaded to: {path}")


def init_dataset(data_info):
    url = data_info['url']
    features_path = data_info['features_path']
    normalized_features_path = data_info['normalized_features_path']
    features_path = DEST_PATH / features_path
    normalized_features_path = DEST_PATH / normalized_features_path
    if os.path.exists(features_path):
        print(f"{features_path} already exists")
        return
    else:
        os.makedirs(os.path.dirname(features_path), exist_ok=True)
        download_dataset(url, features_path)

    if os.path.exists(normalized_features_path):
        print(f"{normalized_features_path} already exists")

        return
    else:
        os.makedirs(os.path.dirname(normalized_features_path), exist_ok=True)
        download_dataset(url, normalized_features_path)


def run():
    # Load JSON file

    # Iterate over each file in the folder
    for filename in os.listdir(SRC_PATH):
        # Construct the full path to the file
        file_path = os.path.join(SRC_PATH, filename)
    
        # Check if the current item is a file
        if os.path.isfile(file_path):
            # Perform actions on the file
            with open(file_path) as f:
                dataset_info = json.load(f)
    
            # setup folders and Download dataset
            init_dataset(dataset_info)
            
    


