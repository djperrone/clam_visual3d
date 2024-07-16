use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
    f32::consts::PI,
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
};

use abd_clam::{graph::Vertex, Cluster};
use csv::{Writer, WriterBuilder};
use distances::Number;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};

use crate::{
    // ffi_impl::cluster_data_wrapper::ClusterDataWrapper,
    graph::force_directed_graph::ForceDirectedGraph,
    utils::types::{Treef32, Vertexf32},
    CBFnNodeVisitorMut,
};

use super::false_nearest_neighbors::FNN_Wrapper;

pub fn choose_two_random_clusters_exclusive<'a, U: Number>(
    clusters: &Vec<&'a Vertex<U>>,
    cluster: &'a Vertex<U>,
) -> Option<[&'a Vertex<U>; 3]> {
    let mut triangle: Vec<&'a Vertex<U>> = Vec::new();
    triangle.push(cluster);
    for c in clusters {
        if triangle.len() < 3 {
            if c != &cluster {
                triangle.push(c);
            }
        } else {
            return Some([triangle[0], triangle[1], triangle[2]]);
        }
    }
    return None;
}
pub fn triangle_from_clusters<'a>(
    tree: &Treef32,
    clusters: &[&'a Vertexf32; 3],
) -> Result<[(&'a str, f32); 3], String> {
    let triangle = [
        (
            "ab",
            clusters[0].distance_to_other(tree.data(), clusters[1]),
        ),
        (
            "ac",
            clusters[0].distance_to_other(tree.data(), clusters[2]),
        ),
        (
            "bc",
            clusters[1].distance_to_other(tree.data(), clusters[2]),
        ),
    ];
    if is_valid_triangle(&triangle) {
        return Ok(triangle);
    } else {
        return Err("Invalid Triangle".to_string());
    }
}

pub fn are_triangles_equivalent(
    clam_edges: &mut [(&str, f32); 3],
    unity_edges: &mut [(&str, f32); 3],
) -> f64 {
    clam_edges.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    unity_edges.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut correct_edge_count = 0;
    for (e1, e2) in clam_edges.iter().zip(unity_edges.iter()) {
        if e1.0 == e2.0 {
            correct_edge_count += 1;
        }
    }

    if correct_edge_count == 3 {
        return 1.0;
    }
    return 0.0;
}

pub fn calc_edge_distortion(
    clam_edges: &mut [(&str, f32); 3],
    unity_edges: &mut [(&str, f32); 3],
) -> f64 {
    // clam_edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    // unity_edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let perimeter_ref: f32 = clam_edges.iter().map(|&(_, value)| value).sum();
    let perimeter_test: f32 = unity_edges.iter().map(|&(_, value)| value).sum();

    let ref_percentages: Vec<f32> = clam_edges
        .iter()
        .map(|&(_, val)| val / perimeter_ref)
        .collect();

    let test_percentages: Vec<f32> = unity_edges
        .iter()
        .map(|&(_, val)| val / perimeter_test)
        .collect();

    let distortion: f32 = ref_percentages
        .iter()
        .zip(test_percentages.iter())
        .map(|(&x, &y)| (y - x).abs())
        .sum();
    return distortion as f64;
}

pub fn calc_angle_distortion(
    clam_edges: &mut [(&str, f32); 3],
    unity_edges: &mut [(&str, f32); 3],
) -> f64 {
    // println!("unity edges");
    let unity_angles = compute_angles_from_edge_lengths(unity_edges);
    // println!("clam edges");

    let clam_angles = compute_angles_from_edge_lengths(clam_edges);
    let ref_angle_sum: f32 = clam_angles.iter().sum();
    let test_angle_sum: f32 = unity_angles.iter().sum();
    let mut err = false;
    if ref_angle_sum > 180. {
        err = true;
        println!("ref angle sum: {}", ref_angle_sum)
    }

    if test_angle_sum > 180. {
        err = true;

        println!("ref angle sum: {}", test_angle_sum)
    }

    // assumes angles sum to 180 - write separate test for this

    let ref_percentages: Vec<f32> = clam_angles.iter().map(|&val| val / ref_angle_sum).collect();

    let test_percentages: Vec<f32> = unity_angles
        .iter()
        .map(|&val| val / test_angle_sum)
        .collect();

    if err {
        println!("ref perc: {:?}", ref_percentages);
        println!("test perc: {:?}", test_percentages);
    }

    let distortion: f32 = ref_percentages
        .iter()
        .zip(test_percentages.iter())
        .map(|(&x, &y)| (y - x).abs())
        .sum();
    if err {
        println!("disortion: {}", distortion);
    }
    return distortion as f64;
}

pub fn compute_angles_from_edge_lengths(edges: &[(&str, f32)]) -> [f32; 3] {
    // println!("{:?}", edges);
    assert!(is_valid_triangle(edges));
    // println!("{:?}", edges);
    // Extract edge lengths for better readability
    let a = edges[0].1;
    let b = edges[1].1;
    let c = edges[2].1;

    // Compute squares of edge lengths for easier calculations
    let a_squared = a * a;
    let b_squared = b * b;
    let c_squared = c * c;

    // Calculate cosines of angles using the law of cosines
    let cosine_a = ((b_squared + c_squared - a_squared) / (2. * b * c)).clamp(-1., 1.);
    let cosine_b = ((a_squared + c_squared - b_squared) / (2. * a * c)).clamp(-1., 1.);
    let cosine_c = ((a_squared + b_squared - c_squared) / (2. * a * b)).clamp(-1., 1.);

    assert!(!cosine_a.is_nan());
    assert!(!cosine_b.is_nan());
    assert!(!cosine_c.is_nan());

    let angle_a = cosine_a.acos();
    let angle_b = cosine_b.acos();
    let angle_c = cosine_c.acos();
    if angle_a.is_nan() {
        println!("cosa: {}", cosine_a);
    }
    assert!(!angle_a.is_nan());
    if angle_b.is_nan() {
        println!("cosb: {}", cosine_b);
    }
    assert!(!angle_b.is_nan());
    if angle_c.is_nan() {
        println!("cosc: {}", cosine_c);
    }
    assert!(!angle_c.is_nan());

    let angle_a = angle_a * 180. / PI;
    let angle_b = angle_b * 180. / PI;
    let angle_c = angle_c * 180. / PI;

    assert!(!angle_a.is_nan());
    assert!(!angle_b.is_nan());
    assert!(!angle_c.is_nan());

    // if angle_a + angle_b + angle_c > 180.0 {
    //     panic!(
    //         "angle sums greater than 180 {}",
    //         angle_a + angle_b + angle_c
    //     )
    // }

    [angle_a, angle_b, angle_c]
}

pub fn get_unity_triangle<'a>(
    clusters: &[&'a Vertexf32; 3],
    fdg: &ForceDirectedGraph,
) -> Result<[(&'a str, f32); 3], String> {
    let [a, b, c] = clusters;
    let unity_a = fdg.get_cluster_position((a.offset(), a.cardinality()))?;
    let unity_b = fdg.get_cluster_position((b.offset(), b.cardinality()))?;
    let unity_c = fdg.get_cluster_position((c.offset(), c.cardinality()))?;

    if are_collinear(unity_a, unity_b, unity_c) {}

    let triangle = [
        ("ab", unity_a.distance(unity_b)),
        ("ac", unity_a.distance(unity_c)),
        ("bc", unity_b.distance(unity_c)),
    ];

    if is_valid_triangle(&triangle) {
        return Ok(triangle);
    } else {
        return Err("invalid triangle".to_string());
    }
}

pub fn write_results(outpath: &PathBuf, results: &Vec<String>) {
    let file = OpenOptions::new().create(true).append(true).open(outpath);
    if let Ok(file) = file {
        // Create a CSV writer
        let mut writer = WriterBuilder::new()
            .delimiter(b',') // Set delimiter (optional)
            .from_writer(file);

        // Write the data to the CSV file
        let _ = writer.write_record(results);

        // Flush and close the writer to ensure all data is written
        let _ = writer.flush();
    }
}

fn is_valid_triangle(edges: &[(&str, f32)]) -> bool {
    if edges.len() != 3 {
        return false;
    }

    let (a, b, c) = (edges[0].1, edges[1].1, edges[2].1);

    // Triangle inequality theorem: the sum of the lengths of any two sides of a triangle must be greater than the length of the third side.
    (a + b > c) && (a + c > b) && (b + c > a)
}

// Function to check if three points are collinear
pub fn are_collinear(p1: glam::Vec3, p2: glam::Vec3, p3: glam::Vec3) -> bool {
    let determinant = p1.x * (p2.y * p3.z - p3.y * p2.z) - p2.x * (p1.y * p3.z - p3.y * p1.z)
        + p3.x * (p1.y * p2.z - p2.y * p1.z);

    determinant.abs() < 1e-6 // Adjust tolerance as needed
}

pub fn read_umap_positions(file_path: &str) -> Result<Vec<glam::Vec3>, Box<dyn Error>> {
    // Open the CSV file
    let file = File::open(Path::new(file_path))?;
    println!("fileapth2 {:?}", file);
    // Create a CSV reader
    let mut rdr = csv::Reader::from_reader(file);

    // Skip the first line (headers)
    rdr.headers().unwrap();

    // Vector to store positions
    let mut positions: Vec<glam::Vec3> = Vec::new();

    // Iterate over each record in the CSV file
    for result in rdr.records() {
        let record = result?;
        // Parse the x, y, z values from the record
        let x: f32 = record[0].parse()?;
        let y: f32 = record[1].parse()?;
        let z: f32 = record[2].parse()?;
        // Create a Position struct and store it in the vector
        positions.push(glam::Vec3::new(x, y, z));
    }

    // Print the positions (for demonstration)
    // println!("Positions: {:?}", positions);

    Ok(positions)
}

pub fn positions_to_distances<'a>(
    positions: &[glam::Vec3; 3],
) -> Result<[(&'a str, f32); 3], String> {
    let triangle = [
        ("ab", positions[0].distance(positions[1])),
        ("ac", positions[0].distance(positions[2])),
        ("bc", positions[1].distance(positions[2])),
    ];

    if is_valid_triangle(&triangle) {
        return Ok(triangle);
    } else {
        return Err("invalid triangle".to_string());
    }
}

pub fn randomly_select_three_indices(
    range: &mut Vec<usize>,
    mut rng: &mut ThreadRng,
) -> (usize, usize, usize) {
    range.partial_shuffle(&mut rng, 3);

    return (range[0], range[1], range[2]);
}

pub fn extract_umap_k(filename: &str) -> Option<u32> {
    // Find the last occurrence of '_'
    if let Some(index) = filename.rfind('n') {
        // Extract the substring from '_' to the end of the string
        let mut substring = filename[index + 1..].to_string();
        substring.truncate(substring.len() - 4);
        println!("subtr {}", substring);
        // Convert the substring to a number
        if let Ok(number) = substring.parse::<u32>() {
            println!("Extracted number: {}", number);
            return Some(number);
        } else {
            println!("Failed to parse the number.");
        }
    } else {
        println!("No '_' found in the string.");
    }
    return None;
}

pub fn write_umap_scores_to_file(
    outpath: &str,
    scores: &HashMap<u32, f64>,
) -> Result<(), Box<dyn Error>> {
    // Open the CSV file for writing
    let file = File::create(outpath)?;
    let mut writer = Writer::from_writer(file);

    // Write keys to the first row
    let keys: Vec<u32> = scores.keys().cloned().collect();
    let keys: Vec<String> = keys.iter().map(|val| val.to_string()).collect();
    writer.write_record(&keys)?;

    // Write values to the second row
    let values: Vec<f64> = scores.values().cloned().collect();
    let values: Vec<String> = values.iter().map(|val| val.to_string()).collect();

    writer.write_record(&values)?;

    writer.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_compute_angles_from_edge_lengths() {
        // Define the edges of the triangle
        let edges: [(&str, f32); 3] = [
            ("a", 3.0), // Length of side 'a'
            ("b", 4.0), // Length of side 'b'
            ("c", 5.0), // Length of side 'c'
        ];

        // Call the function to compute the angles
        let [angle_a, angle_b, angle_c] = compute_angles_from_edge_lengths(&edges);

        // Check if the computed angles are approximately equal to the expected angles
        assert!(approx_eq!(f32, angle_a, 36.8699));
        assert!(approx_eq!(f32, angle_b, 53.1301));
        assert!(approx_eq!(f32, angle_c, 90.0));
    }

    #[test]
    fn test_valid_triangle() {
        // Define edges of a valid triangle
        // let edges_valid: [(&str, f32); 3] = [("a", 3.0), ("b", 4.0), ("c", 5.0)];
        let edges_valid: [(&str, f32); 3] =
            // [("ab", 5.130452), ("ac", 5.307534), ("bc", 0.17708209)];
            // [("ab", 0.0031199455), ("ac", 2.513502), ("bc", 2.510383)];
            [("ab", 4.9336863), ("ac", 4.892655), ("bc", 0.041036364)];

        // Assert that the edges form a valid triangle
        assert!(is_valid_triangle(&edges_valid));

        // // Define edges that do not form a valid triangle (sum of any two edges <= length of the third edge)
        // let edges_invalid: [(&str, f32); 3] = [("a", 1.0), ("b", 2.0), ("c", 5.0)];

        // // Assert that the edges do not form a valid triangle
        // assert!(!is_valid_triangle(&edges_invalid));
    }

    #[test]
    #[test]
    fn test_collinear_points() {
        // Define the coordinates of three points
        let p1 = glam::Vec3::new(0.0, 0.0, 0.0);
        let p2 = glam::Vec3::new(1.0, 1.0, 1.0);
        let p3 = glam::Vec3::new(2.0, 2.0, 2.0);

        // Check if the points are collinear
        assert!(are_collinear(p1, p2, p3));

        // Define coordinates of three non-collinear points
        let p4 = glam::Vec3::new(0.0, 0.0, 0.0);
        let p5 = glam::Vec3::new(1.0, 1.0, 0.0);
        let p6 = glam::Vec3::new(2.0, 2.0, 1.0);

        // Check if the points are not collinear
        assert!(!are_collinear(p4, p5, p6));
    }
}

pub fn binary_heap_to_vec<'a>(
    mut heap: BinaryHeap<FNN_Wrapper<'a, f32>>,
) -> Vec<FNN_Wrapper<'a, f32>> {
    let mut result: Vec<FNN_Wrapper<'a, f32>> = Vec::with_capacity(heap.len());
    while let Some(wrapper) = heap.pop() {
        result.push(wrapper);
    }

    // Since the BinaryHeap pops the largest element first, we need to reverse the vector
    result.reverse();

    result
}

pub fn calc_fnn_scores<'a>(
    original_nn: &HashMap<String, Vec<FNN_Wrapper<'a, f32>>>,
    umap_nn: &HashMap<String, Vec<FNN_Wrapper<'a, f32>>>,
) -> Result<(f64, f64, f64), String> {
    let mut total_precision = 0.0;
    let mut total_recall = 0.0;
    let mut total_f1_score = 0.0;
    let mut count = 0;

    for (idx, original_neighbors) in original_nn.iter() {
        let graph_neighbors = umap_nn.get(idx).unwrap();
        // Compare original_neighbors with graph_neighbors and calculate metrics
        let intersection: usize = original_neighbors
            .iter()
            .filter(|&n| graph_neighbors.contains(&n))
            .count();

        if !original_neighbors.is_empty() && !graph_neighbors.is_empty() {
            let precision = intersection as f64 / original_neighbors.len() as f64;
            let recall = intersection as f64 / graph_neighbors.len() as f64;
            let f1_score = if precision + recall > 0.0 {
                2.0 * (precision * recall) / (precision + recall)
            } else {
                0.0
            };

            total_precision += precision;
            total_recall += recall;
            total_f1_score += f1_score;
            count += 1;
        } else {
            panic!();
        }
    }
    if count > 0 {
        Ok((
            total_precision / count as f64,
            total_recall / count as f64,
            total_f1_score / count as f64,
        ))
    } else {
        // Err("No valid clusters found".to_string())
        panic!();
    }
}
