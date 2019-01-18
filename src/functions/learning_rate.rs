use crate::enums::*;
use crate::core::*;

/// Calculate learning rate
/// 
/// # Arguments
/// 
/// * `lr_type` - Learning rate type
/// * `iteration` - Iteration
/// * `som` - SOM Object
pub fn calc_learning_rate(lr_type: &LearningRateType, iteration: usize, som: &SOM) -> f32 {
  return match lr_type {
    LearningRateType::Constant      => constant_learning_rate(som.learning_rate),
    LearningRateType::Linear        => linear_learning_rate(som.learning_rate, iteration),
    LearningRateType::InverseOfTime => inverse_of_time_learning_rate(som.learning_rate, iteration, som.iterations),
    LearningRateType::PowerSeries   => power_series_learning_rate(som.learning_rate, iteration, som.iterations)
  }
}

/// Constant Learning Rate
fn constant_learning_rate(starting_learning_rate: f32) -> f32 {
  return starting_learning_rate;
}

/// Linear Learning Rate
fn linear_learning_rate(starting_learning_rate: f32, iteration: usize) -> f32 {
  return starting_learning_rate * (1 / iteration) as f32;
}

/// Inverse of Time Learning Rate
fn inverse_of_time_learning_rate(starting_learning_rate: f32, iteration: usize, iteration_limit: usize) -> f32 {
  return starting_learning_rate * (1 - iteration / iteration_limit) as f32;
}

/// Power Series Learning Rate
fn power_series_learning_rate(starting_learning_rate: f32, iteration: usize, iteration_limit: usize) -> f32 {
  return starting_learning_rate * ((iteration / iteration_limit) as f32).exp();
}