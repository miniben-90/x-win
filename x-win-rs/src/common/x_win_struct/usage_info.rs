#![deny(unused_imports)]

/**
 * Struct to store usage data of the window
 */
#[derive(Debug, Clone)]
pub struct UsageInfo {
  pub memory: u32,
}

impl UsageInfo {
  pub fn new(memory: u32) -> Self {
    Self { memory }
  }
}
