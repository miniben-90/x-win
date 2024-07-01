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
