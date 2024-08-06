#![deny(unused_imports)]

/**
 * Struct to store Icon information
 */
#[derive(Debug, Clone)]
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
