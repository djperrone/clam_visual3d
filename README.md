# clam_visual3d
## About
This is my master's thesis I'm working on at the University of Rhode Island. The goal of the project is to use heirarchal clustering to reduce the dimensionality of a dataset for visualization purposes. [CLAM](https://github.com/URI-ABD/clam) is used in the backend to create a tree-like structure where the root node contains all data. It is then recursively split into child nodes that each contain part of the parent's data until some stopping criteria is met. I also build on the work of CHAODA by using it to select a subset of clusters from the tree that represent the dataset as a whole. These selected clusters are used to create a 3D force directed graph that shows the relationship between the clusters and will hopefully provide some insight into the manifold hypothesis that suggests high dimensional data can exist in a lower dimensional space.

## Controls
- When in the visualization, use wasd to move the camera forward, backwards, left, and right
- To move the camera up and down use shift and the space bar, respectively
- Use the mouse to rotate the camera
- The left ctrl key is used to switch between interface modes. One mode allows the camera to move around, the other mode locks the camera and allows the user to interact with the sidebar ui
- `~` opens in-game menu

## Features
- Allows the user to walk through the tree built by clam.
- Clusters can be selected to look at certain properties.
- Different cluster selection functions can be used that can produce different force directed graphs.

## Project Structure
- The backend is implemented in Rust and uses CLAM and CHAODA to build a cluster tree and select a subset of clusters to create the graph.
- The frontend is created using Unity.
- A Foreign Function Interface is used to pass data back and forth from Rust to Unity.

## Project Status
- It should be noted that this is a work in progress.
- If you would like to test the visualization tool, you can clone this repo and run the python build script to create the required Rust library
- You can then run the unity project from the main menu scene and select a dataset to view.
- The dataset needs to be formatted in a particular way and the project currently only supports floating point type data. There are several datasets built into the repo you can use as a demo.
- I am also working on developing CHAODA so the main branch does not currently have a working cluster selection for the graph
- The main branch will have the latest stable features.
