use crate::core::*;
use crate::utils::*;

/// Find Best Matching Unit
/// Returns the node index
/// 
/// # Arguments
/// 
/// * `item` - Data item
/// * `net` - Net object
pub fn find_bmu_index(item: &Vec<isize>, net: &Net) -> usize {
  let mut min_dist: isize = std::isize::MAX;
  let mut bmu_index: usize = 0;

  for (i, node) in net.nodes.iter().enumerate() {
    let dist: isize = squared_euclidean_distance(item, node);
    if dist < min_dist {
      min_dist = dist;
      bmu_index = i;
    }
  }
  return bmu_index;
}

/// Unit tests
#[cfg(test)]
mod functions_tests {
  use crate::core::Net;

  #[test]
  fn test_find_bmu() {
    let mut net: Net = Net::new((2, 2), 3);
    net.nodes = vec![vec![3,4,5], vec![4,5,6], vec![6,7,8], vec![-1,-4,-7]];
    
    let item: Vec<isize> = vec![5,6,7];

    assert_eq!(1, super::find_bmu_index(&item, &net));
  }
}
