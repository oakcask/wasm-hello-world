
mod tests {
  use super::*;

  #[test]
  fn test_create_scale_matrix() {
    assert_eq!(
      create_scale_matrix(12.0, 34.0, 56.0),
      Matrix4(
        12.0, 0.0, 0.0, 0.0,
        0.0, 34.0, 0.0, 0.0,
        0.0, 0.0, 56.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
      ));
  }
}