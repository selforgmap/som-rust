use rayon::prelude::*;

use crate::core::*;
use crate::utils::*;

/// Find Best Matching Unit
/// Returns the node index
/// 
/// # Arguments
/// 
/// * `item` - Data item
/// * `net` - Net object
pub fn find_bmu_index(item: &Vec<f32>, net: &Net) -> usize {
  let nodes = &net.nodes;
  let bmu = nodes.into_par_iter().enumerate()
    .map(|(i, node)| (i, squared_euclidean_distance(&item, &node)))
    .reduce(|| (0usize, std::f32::MAX), |a, b| {
      if a.1 < b.1 { a } else { b }
    });
  return bmu.0;
}

/// Unit tests
#[cfg(test)]
mod functions_tests {
  use crate::core::Net;

  #[test]
  fn test_find_bmu() {
    let mut net: Net = Net::new((2, 2), 3);
    net.nodes = vec![vec![3f32,4f32,5f32], vec![4f32,8f32,6f32], vec![6f32,7f32,8f32], vec![-1f32,-4f32,-7f32]];
    
    let item: Vec<f32> = vec![5f32,6f32,7f32];

    assert_eq!(2, super::find_bmu_index(&item, &net));
  }
}
