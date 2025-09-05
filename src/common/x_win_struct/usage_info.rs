/**
 * Struct to store usage data of the window
 */
#[derive(Debug, Clone)]
#[napi(object)]
pub struct UsageInfo {
  pub memory: u32,
}

impl UsageInfo {
  pub fn new(memory: u32) -> Self {
    Self { memory }
  }
}

impl From<x_win::UsageInfo> for UsageInfo {
  fn from(value: x_win::UsageInfo) -> Self {
    UsageInfo {
      memory: value.memory,
    }
  }
}

impl From<UsageInfo> for x_win::UsageInfo {
  fn from(value: UsageInfo) -> Self {
    x_win::UsageInfo {
      memory: value.memory,
    }
  }
}
