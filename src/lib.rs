mod enums;

pub use crate::enums::*;

// SOM Struct
pub struct SOM {
  pub size                  : (i32, i32),
  pub dimension             : i32,
  pub grid_type             : GridType,
  pub learning_rate         : f32,
  pub learning_rate_function: LearningRateFunction,
  pub initializing_method   : InitializingMethod,
  pub neighborhood_function : NeighborhoodFunction,
  pub iterations            : i32
}

impl SOM {
  pub fn test() {
    println!("hello")
  }
}
