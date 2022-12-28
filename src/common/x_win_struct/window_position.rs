#![deny(unused_imports)]

/**
 * Struct to store position and size of the window
 */
#[derive(Debug, Clone)]
#[napi(object)]
pub struct WindowPosition {
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
}

impl WindowPosition {
  pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
    Self {
      x,
      y,
      width,
      height,
    }
  }
}