#![deny(unused_imports)]

use super::{process_info::ProcessInfo, usage_info::UsageInfo, window_position::WindowPosition};

/**
 * Struct to store all informations of the window
 */
#[derive(Debug, Clone)]
pub struct WindowInfo {
  pub id: u32,
  pub os: String,
  pub title: String,
  pub position: WindowPosition,
  pub info: ProcessInfo,
  pub usage: UsageInfo,
}

impl WindowInfo {
  pub fn new(
    id: u32,
    os: String,
    title: String,
    position: WindowPosition,
    info: ProcessInfo,
    usage: UsageInfo,
  ) -> Self {
    Self {
      id,
      os,
      title,
      position,
      info,
      usage,
    }
  }
}
