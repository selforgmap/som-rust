use crate::enums::*;

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

  // Start train
  pub fn train(&self, dataset: &Vec<&Vec<i32>>, iterations: i32) -> () {
    println!("Traing {} {}", dataset[1][2], iterations);
  }
}