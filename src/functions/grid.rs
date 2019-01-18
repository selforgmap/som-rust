use crate::enums::*;
use crate::utils::*;

// Matrix definitions
pub type DistanceMatrix = Vec<Vec<f32>>;
pub type LocationMatrix = Vec<(f32, f32)>;

/// Generate Location Matrix
/// 
/// # Arguments
/// 
/// * `grid_type` - Grid type
/// * `size` - Width and Height of neural net
pub fn generate_location_matrix(grid_type: &GridType, size: (usize, usize)) -> LocationMatrix {
  let mut loc_matrix: LocationMatrix = LocationMatrix::new();

  match grid_type {
    // Rectangular Grid
    GridType::Rectangular => {
      for x in 0..size.0 {
        for y in 0..size.1 {
          loc_matrix.push((x as f32, y as f32));
        }
      }
    }
    // Hexagonal Grid
    GridType::Hexagonal => {

    }
  }

  return loc_matrix;
}

/// Generate Distance Matrix
/// 
/// # Arguments
/// 
/// * `loc_matrix` - Location matrix
/// * `size` - Width and Height of neural net
pub fn generate_distance_matrix(loc_matrix: &LocationMatrix, size: (usize, usize)) -> DistanceMatrix {
  let node_count: usize = size.0 * size.1;
  let mut dist_matrix: DistanceMatrix = vec![vec![0f32; node_count]; node_count];

  // For each nodes
  for a in 0..node_count {
    for b in 0..node_count {
      let loc_a = loc_matrix[a];
      let loc_b = loc_matrix[b];
      dist_matrix[a][b] = euclidean_distance(loc_a, loc_b);
    }
  }

  return dist_matrix;
}