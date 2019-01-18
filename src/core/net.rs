use std::fmt;

/// Net struct : Neural network
pub struct Net {
  pub nodes: Vec<Vec<f32>>,
  pub size: (usize, usize)
}

impl Net {
  /// Create new Net instance
  pub fn new(size: (usize, usize), dimension: usize) -> Net {
    Net {
      nodes: vec![vec![0f32; dimension as usize]; (size.0 * size.1) as usize],
      size: size
    }
  }

  /// To String
  pub fn to_string(&self) -> String {
    let list: Vec<String> = self.nodes.chunks(self.size.0)
      .map(|line| format!("{:.2?}", line))
      .collect();
    return list.join("\n");
  }
}

impl fmt::Display for Net {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}
