use rayon::prelude::*;


/// Calculate Squared Euclidean Distance
/// 
/// # Arguments
/// 
/// * `first` - Vector 1
/// * `second` - Vector 2
pub fn squared_euclidean_distance(first: &Vec<f32>, second: &Vec<f32>) -> f32 {
  return first.into_par_iter().zip(second).into_par_iter()
    .map(|(a, b)| (a - b).powi(2))
    .sum()
}

/// Calculate Euclidean Distance
/// 
/// # Arguments
/// 
/// * `first` - Location 1
/// * `second` - Location 2
pub fn euclidean_distance(first: (f32, f32), second: (f32, f32)) -> f32 {
  return ((first.0 - second.0).powi(2) + (first.1 - second.1).powi(2)).sqrt();
}


/// Unit tests
#[cfg(test)]
mod math_tests {
  #[test]
  fn test_squared_euclidean_distance() {
    let a: Vec<f32> = vec![2f32, 3f32, 4f32];
    let b: Vec<f32> = vec![6f32, 5f32, 2f32];
    assert_eq!(24f32, super::squared_euclidean_distance(&a, &b));
  }

  #[test]
  fn test_euclidean_distance() {
    assert_eq!(5f32, super::euclidean_distance((2.5f32, 3f32), (6.5f32, 6f32)))
  }
}