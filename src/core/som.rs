use crate::enums::*;
use crate::core::*;

// SOM Struct
pub struct SOM {
  pub size                  : (u32, u32),
  pub grid_type             : GridType,
  pub learning_rate         : f32,
  pub learning_rate_function: LearningRateFunction,
  pub initializing_method   : InitializingMethod,
  pub neighborhood_function : NeighborhoodFunction,
  pub iterations            : i32,
}

impl SOM {
  // New som
  pub fn new(size: (u32, u32)) -> SOM {
    SOM {
      size                  : size,
      grid_type             : GridType::Rectangular,
      learning_rate         : 0.1,
      learning_rate_function: LearningRateFunction::Constant,
      initializing_method   : InitializingMethod::Random,
      neighborhood_function : NeighborhoodFunction::Bubble,
      iterations            : 100
    }
  }

  // Start training
  pub fn train(&self, dataset: &Vec<&Vec<i32>>, iterations: i32) -> () {
    let dimension: u32 = 3; // Dimension
    let _net: Net = Net::new(self.size, dimension);

    // For each iteration
    for _i in 0..iterations {
      // Foe each dataitem
      for _item in dataset {

        // println!("{}", item[0])
      }

      // println!("{}", i);
    }






    println!("Traing {} {}", dataset[1][2], iterations);


  }
}