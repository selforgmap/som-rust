/// Net struct : Neural network
pub struct Net {
  pub nodes: Vec<Vec<isize>>,
  pub size: (usize, usize)
}

impl Net {
  /// Create new Net instance
  pub fn new(size: (usize, usize), dimension: usize) -> Net {
    Net {
      nodes: vec![vec![0isize; dimension as usize]; (size.0 * size.1) as usize],
      size: size
    }
  }
}