use crate::mat4;

use super::Matrix4;

#[test]
fn test_scaling() {
  assert_eq!(
    Matrix4::scaling(12.0, 34.0, 56.0),
    mat4!(
      12.0, 0.0, 0.0, 0.0,
      0.0, 34.0, 0.0, 0.0,
      0.0, 0.0, 56.0, 0.0,
      0.0, 0.0, 0.0, 1.0
    ));
}