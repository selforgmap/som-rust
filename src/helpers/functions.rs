use crate::core::Net;
use crate::helpers::math::*;

// Find BMU
pub fn find_bmu(item: &Vec<i32>, net: Net) -> u32 {
  let mut min_dist: i32 = std::i32::MAX;
  let mut bmu: u32 = 0;

  for (i, node) in net.nodes.iter().enumerate() {
    let dist: i32 = squared_euclidean_distance(item, node);
    if dist < min_dist {
      min_dist = dist;
      bmu = i as u32;
    }
  }
  return bmu;
}



// Unit tests
#[cfg(test)]
mod functions_tests {
  use crate::core::Net;

  #[test]
  fn test_find_bmu() {
    let mut net: Net = Net::new((2, 2), 3);
    net.nodes = vec![vec![3,4,5], vec![4,5,6], vec![6,7,8], vec![-1,-4,-7]];
    
    let item: Vec<i32> = vec![5,6,7];

    assert_eq!(1, super::find_bmu(&item, net));
  }
}
