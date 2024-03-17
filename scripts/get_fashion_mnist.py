import numpy as np
import tensorflow as tf
import pathlib
import platform
import numpy as np

PROJECT_PATH = pathlib.Path(__file__).parent.parent.resolve()

DATA_PATH = PROJECT_PATH / "data/anomaly_data/preprocessed"
DATA_PATH.mkdir(exist_ok=True)


# Load Fashion MNIST dataset
fashion_mnist = tf.keras.datasets.fashion_mnist
(train_images, train_labels), (test_images, test_labels) = fashion_mnist.load_data()

# Concatenate training and test data
# images = np.concatenate((train_images, test_images))
# labels = np.concatenate((train_labels, test_labels))

images = train_images.astype(np.float32) / np.float32(255.0)

# Convert labels to integers
labels = train_labels.astype(np.uint8)

print("Finished converting. Saving features and scores")

# Save features and labels into .npy files
features_path = DATA_PATH / 'fashion-mnist_features.npy'
scores_path = DATA_PATH / 'fashion-mnist_scores.npy'

print("Dimensions of images:", images.shape)
print("Dimensions of labels:", labels.shape)
num_images = images.shape[0]
image_size = images.shape[1] * images.shape[2]  # 28x28 = 784
images_2d = images.reshape(num_images, image_size)

# Print dimensions of flattened images
print("Dimensions of flattened images:", images_2d.shape)

# Save features (images) and labels into .npy files
np.save(features_path, images_2d)
np.save(scores_path, labels)

print("Fashion MNIST dataset has been saved.")
