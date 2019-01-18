use crate::core::*;
use crate::enums::*;
use crate::functions::*;


/// SOM Struct
pub struct SOM {
  pub size                  : (usize, usize),
  pub grid_type             : GridType,
  pub learning_rate         : f32,
  pub learning_rate_type    : LearningRateType,
  pub neighborhood_type     : NeighborhoodType,
  pub initializing_method   : InitializingMethod,
  pub iterations            : usize,
}

impl SOM {
  /// Create new instance
  /// 
  /// # Arguments
  /// 
  /// * `size` - Tuple of width and height
  pub fn new(size: (usize, usize)) -> SOM {
    SOM {
      size                  : size,
      grid_type             : GridType::Rectangular,
      learning_rate         : 0.1,
      learning_rate_type    : LearningRateType::Constant,
      neighborhood_type     : NeighborhoodType::Bubble,
      initializing_method   : InitializingMethod::Random,
      iterations            : 100
    }
  }

  /// Start training
  /// 
  /// # Arguments
  /// 
  /// * `dataset` - Input dataset
  /// * `iterations` - No of iterations
  pub fn train(&self, dataset: &Vec<&Vec<isize>>, iterations: isize) -> () {
    let dimension: usize = 3; // Dimension
    let net: Net = Net::new(self.size, dimension);

    let loc_matrix:  LocationMatrix = generate_location_matrix(&self.grid_type, self.size);
    let dist_matrix: DistanceMatrix = generate_distance_matrix(&loc_matrix, self.size);

    // For each iteration
    for iter in 0..iterations {
      // Foe each dataitem
      for item in dataset {

        // Calculate BMU Index
        let bmu_index: usize = find_bmu_index(&item, &net);

        // Cooperative Process
        let _learning_rate = calc_learning_rate(&self.learning_rate_type, iter as usize, self);

        // Adopt neighbors
        for (node_index, _node) in net.nodes.iter().enumerate() {
          // Calculate Neighborhood Value
          let _nh_value: f32 = calc_neighborhood_value(&self.neighborhood_type, bmu_index, node_index, iter as usize, &dist_matrix);

        }

      }

    }

    println!("Traing {} {}", dataset[1][2], iterations);
  }
}