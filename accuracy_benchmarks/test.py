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

data = {
    'X': [10, 11, 4, 5, 6, 7, 8, 9],
    'Label': ['edge_equivalence', 'edge_equivalence', 'edge_equivalence', 'edge_equivalence', 'edge_equivalence', 'edge_equivalence', 'edge_equivalence', 'edge_equivalence', 
              'edge_distortion', 'edge_distortion', 'edge_distortion', 'edge_distortion', 'edge_distortion', 'edge_distortion', 'edge_distortion', 'edge_distortion',
              'angle_distortion', 'angle_distortion', 'angle_distortion', 'angle_distortion', 'angle_distortion', 'angle_distortion', 'angle_distortion', 'angle_distortion'],
    'Value': [0.652000, 0.670690, 0.866667, 0.746154, 0.792308, 0.678125, 0.822222, 0.664000,
              0.144901, 0.169386, 0.074026, 0.103766, 0.105268, 0.112281, 0.151487, 0.147076,
              0.301839, 0.357984, 0.244047, 0.266858, 0.294845, 0.275559, 0.306916, 0.318371]
}

# Create DataFrame
df = pd.DataFrame(data)

# Plot the DataFrame
fig, ax = plt.subplots()

# Group data by 'Label' and plot
for label, group in df.groupby('Label'):
    group.plot(x='X', y='Value', ax=ax, label=label, kind='bar', alpha=0.8)

# Add title and labels
plt.title('Example Plot')
plt.xlabel('X')
plt.ylabel('Value')

# Save plot to image file
plt.savefig('plot.png')