use crate::enums::*;
use crate::functions::*;

/// Calculate Neighborhood Value
/// 
/// # Arguments
/// 
/// * `nh_type` - Neighborhood type
/// * `bmu_index` - Index of the BMU node
/// * `node_index` - Index of the node
/// * `iteration` - Current Iteration
/// * `dist_matrix` = Distance matrix
pub fn calc_neighborhood_value(nh_type: &NeighborhoodType, bmu_index: usize, node_index: usize, iteration: usize, dist_matrix: &DistanceMatrix) -> f32 {
  return match nh_type {
    NeighborhoodType::Bubble => bubble_neighborhood(bmu_index, node_index, iteration, dist_matrix)
  }
}

/// Bubble Neighborhood
fn bubble_neighborhood(bmu_index: usize, node_index: usize, _iteration: usize, dist_matrix: &DistanceMatrix) -> f32 {
  // TODO
  if dist_matrix[bmu_index][node_index] > 2f32 {
    return 0f32;
  } else {
    return 1f32;
  }
}