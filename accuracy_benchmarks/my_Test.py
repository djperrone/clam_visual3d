import matplotlib.pyplot as plt
import numpy as np

# Sample data
x = np.linspace(0, 10, 100)
y1 = np.sin(x)
y2 = np.cos(x)

# Create subplots
fig, axs = plt.subplots(1, 2, figsize=(10, 5))  # 1 row, 2 columns

# Plot data on the first subplot
axs[0].plot(x, y1, color='blue')
axs[0].set_title('Sine Function')

# Plot data on the second subplot
axs[1].plot(x, y2, color='red')
axs[1].set_title('Cosine Function')

# Adjust layout
plt.tight_layout()

# Save the figure
plt.savefig('two_plots_side_by_side.png')

# Show the figure
plt.show()