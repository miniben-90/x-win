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
  pub is_full_screen: bool,
}

impl WindowPosition {
  pub fn new(x: i32, y: i32, width: i32, height: i32, is_full_screen: bool) -> Self {
    Self {
      x,
      y,
      width,
      height,
      is_full_screen,
    }
  }
}

impl From<x_win::WindowPosition> for WindowPosition {
  fn from(value: x_win::WindowPosition) -> Self {
    WindowPosition {
      x: value.x,
      y: value.y,
      width: value.width,
      height: value.height,
      is_full_screen: value.is_full_screen,
    }
  }
}

impl From<WindowPosition> for x_win::WindowPosition {
  fn from(value: WindowPosition) -> Self {
    x_win::WindowPosition {
      x: value.x,
      y: value.y,
      width: value.width,
      height: value.height,
      is_full_screen: value.is_full_screen,
    }
  }
}
