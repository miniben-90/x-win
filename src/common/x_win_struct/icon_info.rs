#![deny(unused_imports)]

/**
 * Struct to store process information of the window
 */
#[derive(Debug, Clone)]
#[napi(object)]
#[repr(C)]
pub struct IconInfo {
  pub data: String,
  pub height: u32,
  pub width: u32,
}

impl IconInfo {
  pub fn new(data: String, height: u32, width: u32) -> Self {
    Self {
      data,
      height,
      width,
    }
  }
}

impl From<x_win::IconInfo> for IconInfo {
  fn from(value: x_win::IconInfo) -> Self {
    IconInfo {
      data: value.data,
      height: value.height,
      width: value.width,
    }
  }
}

impl From<IconInfo> for x_win::IconInfo {
  fn from(value: IconInfo) -> Self {
    x_win::IconInfo {
      data: value.data,
      height: value.height,
      width: value.width,
    }
  }
}
