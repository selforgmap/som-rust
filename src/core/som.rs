use crate::core::*;
use crate::enums::*;
use crate::functions::*;


// SOM Struct
pub struct SOM {
  pub size                  : (u32, u32),
  pub grid_type             : GridType,
  pub learning_rate         : f32,
  pub learning_rate_type    : LearningRateType,
  pub neighborhood_type     : NeighborhoodType,
  pub initializing_method   : InitializingMethod,
  pub iterations            : i32,
}

impl SOM {
  // New som
  pub fn new(size: (u32, u32)) -> SOM {
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

  // Start training
  pub fn train(&self, dataset: &Vec<&Vec<i32>>, iterations: i32) -> () {
    let dimension: u32 = 3; // Dimension
    let net: Net = Net::new(self.size, dimension);

    // For each iteration
    for _i in 0..iterations {
      // Foe each dataitem
      for item in dataset {

        let _bmu: u32 = find_bmu(&item, &net);

      }

    }

    println!("Traing {} {}", dataset[1][2], iterations);
  }
}