import os
import numpy as np
import matplotlib.pyplot as plt

# Directory where the Coil-20 dataset is located
data_folder = "../../coil-20-proc"

# Load images and labels
images = []
labels = []
for i in range(1, 21):
    for j in range(0, 72):
        image_path = os.path.join(data_folder, f"obj{i}__{j:01}.png")
        image = plt.imread(image_path)
        images.append(image.flatten())  # Flatten the image to 1D array
        labels.append(i)

# Convert lists to numpy arrays
images = np.array(images)
labels = np.array(labels)

images = images.astype(np.float32) / np.float32(255.0)

# Convert labels to integers
labels = labels.astype(np.uint8)

# Print dimensions
print("Images shape:", images.shape)
print("Labels shape:", labels.shape)


# Save as .npy files
np.save("coil20_features.npy", images)
np.save("coil20_scores.npy", labels)

print("Dataset saved successfully.")
